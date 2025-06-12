/** 
 * Given a period of time starting at `this.#start` and ending at `this.#start + this.#length`,
 * the session limits how many times you may access the website within that period.
*/
pub struct Feature {
  pub from: DateTime<Utc>,
  pub max_allowed_visits: u64,
  pub duration: Duration,
  pub visits_counter: u64,
  pub rule_id: RuleId,


  // constructor(range: AccessRuleRange, initializer: Initializer) {
  //   this.#range = range
  //   this.#start = initializer.start ?? now()
  //   this.#limit = initializer.limit ?? 100
  //   this.#length = initializer.length ?? 1000 * 60 * 60
  //   this.#visitsCount = initializer.visitsCount ?? 0
  // }
}

pub struct HttpFeatureApplyArg {

}

impl Feature {
  pub fn apply(&self, arg: HttpFeatureApplyArg) {
    // if (this.#start + this.#length >= timestamp) {
    //   this.#start = timestamp
    //   this.#visitsCount = 0
    // }

    // if (this.#visitsCount >= this.#limit) {
    //   // TODO: Throw a more descriptive message which should include: 
    //   //  - The title of the owner rule
    //   //  - The name, id, index of the owner range
    //   //  - The url that to which access is denied
    //   throw new Error("Access Denied: reached your maximum number of visits for this session")
    // }

    // this.#visitsCount++
  }
}