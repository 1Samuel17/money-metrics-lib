// Integration Tests

#[cfg(test)]
mod tests {
    use money_metrics_lib::{
        incomes::{Income, Incomes},
        obligations::{Obligation, Obligations},
        scenarios::Scenario,
        utilities::Frequency,
    };

    #[test]
    fn test_workflow_integration() {
        // create default incomes collection
        let mut default_incomes = Incomes::new();
        // create default obligations collection
        let mut default_obligations = Obligations::new();
        // create incomes
        let dayjob = Income::Employer {
            description: (String::from("day job")),
            amount: (1500.00),
            frequency: (Frequency::Biweekly),
            note: (String::from("40hr week")),
        };
        let side_hustle = Income::Project {
            description: (String::from("side hustle")),
            amount: (200.00),
            frequency: (Frequency::Random),
            note: (String::from("approximate payout")),
        };
        let rental_income = Income::Property {
            description: (String::from("rv renting")),
            amount: (300.00),
            frequency: (Frequency::Monthly),
            note: (String::from("passthrough income")),
        };
        // add incomes to default incomes collection
        let _ = &default_incomes.push(dayjob);
        let _ = &default_incomes.push(side_hustle);
        let _ = &default_incomes.push(rental_income);
        // create obligations
        let mortgage = Obligation::Mortgage {
            amount: (2500.00),
            frequency: (Frequency::Monthly),
            note: (String::from("escrow included")),
        };
        let electricity = Obligation::Electricity {
            amount: (350.00),
            frequency: (Frequency::Monthly),
            note: (String::from("summer estimate")),
        };
        let water = Obligation::Water {
            amount: (80.00),
            frequency: (Frequency::Monthly),
            note: (String::from("high estimate")),
        };
        // add obligations to the default obligations collection
        let _ = &default_obligations.push(mortgage);
        let _ = &default_obligations.push(electricity);
        let _ = &default_obligations.push(water);
        // create scenario
        let default_scenario = Scenario::Hypothetical {
            incomes: (default_incomes),
            obligations: (default_obligations),
            note: (String::from("default scenario")),
        };

        println!("{}", default_scenario);

        match default_scenario {
            Scenario::Hypothetical {
                incomes,
                obligations,
                note,
            } => {
                assert_eq!(incomes.len(), 3);
                assert_eq!(obligations.len(), 3);
                assert_eq!(note, "default scenario");
            }
            _ => panic!("something went wrong"),
        }
    }
}
