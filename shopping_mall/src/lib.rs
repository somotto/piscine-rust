pub use crate::mall::*;
pub use crate::mall::floor::Floor;
pub use crate::mall::floor::store::Store;
pub use crate::mall::floor::store::employee::Employee;
pub use crate::mall::guard::Guard;


pub fn biggest_store(mall: Mall) -> Store {
    mall.floors
        .iter()
        .flat_map(|floor| floor.stores.clone())
        .max_by_key(|store| store.square_meters)
        .unwrap()
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let all_employees: Vec<Employee> = mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .flat_map(|store| store.employees.clone())
        .collect();

    let max_salary = all_employees
        .iter()
        .map(|e| e.salary)
        .fold(0.0, f64::max);

    all_employees
        .into_iter()
        .filter(|e| (e.salary - max_salary).abs() < f64::EPSILON)
        .collect()
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    let store_employees: usize = mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .map(|store| store.employees.len())
        .sum();

    store_employees + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, mut new_guards: Vec<Guard>) {
    let total_space: u64 = mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .map(|store| store.square_meters)
        .sum();

    let required_guards = (total_space + 199) / 200; 

    let current_guard_count = mall.guards.len();
    let needed = required_guards as isize - current_guard_count as isize;

    if needed > 0 {
        let to_add = needed.min(new_guards.len() as isize) as usize;
        mall.guards.extend(new_guards.drain(..to_add));
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for employee in &mut store.employees {
                let (start, end) = employee.working_hours;
                let hours = end.saturating_sub(start) as f64;

                if hours > 10.0 {
                    employee.salary *= 1.10;
                } else {
                    employee.salary *= 0.90;
                }
            }
        }
    }
}