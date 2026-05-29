use crate::utilities::Frequency;

// TODO:

#[derive(Debug, PartialEq, Eq)]
pub struct Paycheck {
    pub paycheck_scenario: PaycheckScenario,
    pub salary: Option<u32>,          // if salary
    pub hourly_rate: Option<u32>,     // if hourly
    pub hours_worked: Option<u32>,    // per week
    pub pretax_deductions: Vec<u32>,  // fix: use struct enum list instead
    pub posttax_deductions: Vec<u32>, // fix: use struct enum list instead
    pub withholdings: Vec<u32>,       // fix: use struct enum list instead
    pub pay_frequency: Frequency,
}

impl Paycheck {
    pub fn new(
        paycheck_scenario: PaycheckScenario,
        salary: Option<u32>,
        hourly_rate: Option<u32>,
        hours_worked: Option<u32>,
        pretax_deductions: Vec<u32>,
        posttax_deductions: Vec<u32>,
        withholdings: Vec<u32>,
        pay_frequency: Frequency,
    ) -> Self {
        Self {
            paycheck_scenario,
            salary,
            hourly_rate,
            hours_worked,
            pretax_deductions,
            posttax_deductions,
            withholdings,
            pay_frequency,
        }
    }

    pub fn get_salary() {
        // get salary from hourly
    }

    pub fn get_hourly() {
        // get hourly from salary
    }

    pub fn get_biweekly() {
        // get biweekly from salary
    }

    pub fn get_monthly() {
        // get monthly pay from salary or hourly
    }

    pub fn calculate_gross_paycheck() {
        // calculate gross paycheck from hours worked per week and hourly rate
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum PaycheckScenario {
    #[default]
    Real,
    Hypothetical,
}
