use std::{thread, time::Duration};

fn main() {}

enum HealthIndicator {
    VeryGood,
    Good,
    Okay,
    Sick,
}

enum AgeGroup {
    Kid,
    YoungAdault,
    Adault,
    MiddleAged,
    Senior,
    SuperSenior,
}

enum WorkoutPreference {}

enum BodyMassIndexLevel {}

enum IntensityLevel {}

struct WorkoutCriteria {
    agegoup_indicator: AgeGroup,
    health_indicator: HealthIndicator,
}

struct Workout {
    walking: Duration,
    running: Duration,
    situps: u8,
    pushups: u8,
}

impl Workout {
    fn new() -> Workout {
        Workout {
            walking: Duration::from_secs(60 * 60),
            running: Duration::from_secs(60 * 30),
            situps: 50,
            pushups: 50,
        }
    }
}

fn evaluage_agegroup_criteria(age: u8) -> AgeGroup {
    let agegroup;

    if age <= 10 {
        agegroup = AgeGroup::Kid;
    } else if age >= 11 && age <= 17 {
        agegroup = AgeGroup::YoungAdault;
    } else if age >= 18 && age <= 45 {
        agegroup = AgeGroup::Adault;
    } else if age >= 45 && age <= 59 {
        agegroup = AgeGroup::MiddleAged;
    } else if age >= 60 && age <= 75 {
        agegroup = AgeGroup::Senior;
    } else {
        agegroup = AgeGroup::SuperSenior;
    }

    agegroup
}

fn workout_calculation(criteria: WorkoutCriteria) -> Workout {
    println!("This method takes 2 seconds...");

    thread::sleep(Duration::from_secs(2));
}

fn generate_workout(intensity: u32) {}
