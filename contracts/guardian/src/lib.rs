use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault, Promise, PromiseOrValue,
};

/// Simple policy parameters for an agent
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Policy {
    /// Maximum amount (in yoctoNEAR) allowed in a single transfer
    pub max_single_transfer: u128,
    /// Maximum total outgoing volume in a rolling window (simplified for MVP)
    pub max_daily_volume: u128,
    /// Whether the agent is currently frozen
    pub is_frozen: bool,
    /// Optional: only allow transfers to these accounts (empty = no restriction)
    // For MVP we keep it simple — full whitelist support can come later
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            max_single_transfer: 10_000_000_000_000_000_000_000_000, // 10 NEAR
            max_daily_volume: 50_000_000_000_000_000_000_000_000,   // 50 NEAR
            is_frozen: false,
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Guardian {
    /// Owner / admin of the Guardian (can update policies, force freeze, etc.)
    pub owner: AccountId,
    /// Registered agents and their policies
    pub agents: LookupMap<AccountId, Policy>,
    /// Set of registered agent account IDs (for easy listing)
    pub agent_ids: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Guardian {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            owner,
            agents: LookupMap::new(b"a"),
            agent_ids: UnorderedSet::new(b"i"),
        }
    }

    /// Register a new agent under protection
    pub fn register_agent(&mut self, agent_id: AccountId, policy: Option<Policy>) {
        self.assert_owner();
        assert!(!self.agents.contains_key(&agent_id), "Agent already registered");

        let policy = policy.unwrap_or_default();
        self.agents.insert(&agent_id, &policy);
        self.agent_ids.insert(&agent_id);

        env::log_str(&format!("Registered agent: {}", agent_id));
    }

    /// Update policy for an existing agent
    pub fn update_policy(&mut self, agent_id: AccountId, policy: Policy) {
        self.assert_owner();
        assert!(self.agents.contains_key(&agent_id), "Agent not registered");

        self.agents.insert(&agent_id, &policy);
        env::log_str(&format!("Updated policy for agent: {}", agent_id));
    }

    /// Freeze an agent (blocks outgoing transfers if integrated)
    pub fn freeze_agent(&mut self, agent_id: AccountId) {
        self.assert_owner_or_watcher(); // for MVP we allow owner; later watcher role

        let mut policy = self
            .agents
            .get(&agent_id)
            .expect("Agent not registered");

        policy.is_frozen = true;
        self.agents.insert(&agent_id, &policy);

        env::log_str(&format!("FROZEN agent: {}", agent_id));
    }

    /// Unfreeze an agent
    pub fn unfreeze_agent(&mut self, agent_id: AccountId) {
        self.assert_owner();

        let mut policy = self
            .agents
            .get(&agent_id)
            .expect("Agent not registered");

        policy.is_frozen = false;
        self.agents.insert(&agent_id, &policy);

        env::log_str(&format!("Unfrozen agent: {}", agent_id));
    }

    /// Check whether a proposed transfer is allowed
    /// Returns true if allowed, false if it should be blocked
    pub fn is_transfer_allowed(
        &self,
        agent_id: AccountId,
        amount: U128,
        _receiver_id: AccountId,
    ) -> bool {
        let policy = match self.agents.get(&agent_id) {
            Some(p) => p,
            None => return true, // not under protection → allow
        };

        if policy.is_frozen {
            return false;
        }

        if amount.0 > policy.max_single_transfer {
            return false;
        }

        // TODO: velocity / daily volume tracking (requires more storage)
        true
    }

    /// View helpers
    pub fn get_policy(&self, agent_id: AccountId) -> Option<Policy> {
        self.agents.get(&agent_id)
    }

    pub fn is_frozen(&self, agent_id: AccountId) -> bool {
        self.agents
            .get(&agent_id)
            .map(|p| p.is_frozen)
            .unwrap_or(false)
    }

    pub fn get_registered_agents(&self) -> Vec<AccountId> {
        self.agent_ids.to_vec()
    }

    // ─── Internal helpers ───

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "Only owner can call this method"
        );
    }

    // For MVP we treat owner as the only privileged caller.
    // Later we can add a dedicated "watcher" role.
    fn assert_owner_or_watcher(&self) {
        self.assert_owner();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn test_register_and_freeze() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = Guardian::new(accounts(0));

        contract.register_agent(accounts(1), None);
        assert!(!contract.is_frozen(accounts(1)));

        contract.freeze_agent(accounts(1));
        assert!(contract.is_frozen(accounts(1)));

        assert!(!contract.is_transfer_allowed(
            accounts(1),
            U128(1_000_000_000_000_000_000_000_000), // 1 NEAR
            accounts(2)
        ));
    }
}
