use crate::types::TimestampMillis;
// use candid::Principal;

pub trait Environment {
    fn now(&self) -> TimestampMillis;
    // fn caller(&self) -> Principal;
}

pub struct CanisterEnvironment {}

impl CanisterEnvironment {
    pub fn new() -> Self {
        CanisterEnvironment {}
    }
}

impl Environment for CanisterEnvironment {
    fn now(&self) -> u64 {
        ic_cdk::api::time()
    }

    // fn caller(&self) -> Principal {
    //     ic_cdk::caller()
    // }
}

pub struct EmptyEnvironment {}

impl EmptyEnvironment {
    pub fn new() -> Self {
        EmptyEnvironment {}
    }
}

impl Environment for EmptyEnvironment {
    fn now(&self) -> u64 {
        0
    }

    // fn caller(&self) -> Principal {
    //     unimplemented!()
    // }
}

#[cfg(test)]
pub struct TestEnvironment {
    pub now: u64,
    // pub caller: Principal,
}

#[cfg(test)]
impl TestEnvironment {
    // pub fn new(now: u64, caller: Principal) -> Self {
    //     TestEnvironment { now, caller }
    // }
    pub fn new(now: u64) -> Self {
        TestEnvironment { now }
    }
}

#[cfg(test)]
impl Environment for TestEnvironment {
    fn now(&self) -> u64 {
        self.now
    }

    // fn caller(&self) -> Principal {
    //     self.caller
    // }
}
