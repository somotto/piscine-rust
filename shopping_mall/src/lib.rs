fn biggest_store(mall: mall::Mall) -> mall::floor::store::Store {
    let mut biggest = None;
    
    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            match biggest {
                None => biggest = Some(store),
                Some(current_biggest) => {
                    if store.square_meters > current_biggest.square_meters {
                        biggest = Some(store);
                    }
                }
            }
        }
    }
    
    biggest.unwrap().clone()
}

fn highest_paid_employee(mall: mall::Mall) -> Vec<mall::floor::store::employee::Employee> {
    let mut highest_salary = 0.0;
    let mut highest_paid = Vec::new();
    
    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            for employee in store.employees.iter() {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                    highest_paid.clear();
                    highest_paid.push(employee.clone());
                } else if employee.salary == highest_salary {
                    highest_paid.push(employee.clone());
                }
            }
        }
    }
    
    highest_paid
}

fn nbr_of_employees(mall: mall::Mall) -> usize {
    let mut count = mall.guards.len();
    
    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            count += store.employees.len();
        }
    }
    
    count
}

fn check_for_securities(mall: &mut mall::Mall, available_guards: Vec<mall::guard::Guard>) {
    let mut total_square_meters = 0;
    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            total_square_meters += store.square_meters;
        }
    }
    
    let needed_guards = (total_square_meters + 199) / 200; 
    let current_guards = mall.guards.len();
    
    if needed_guards > current_guards {
        let guards_to_add = needed_guards - current_guards;
        
        let available_count = available_guards.len().min(guards_to_add);
        
        for i in 0..available_count {
            mall.hire_guard(available_guards[i].clone());
        }
    }
}

fn cut_or_raise(mall: &mut mall::Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for employee in &mut store.employees {
                let (entry, exit) = employee.working_hours;
                let hours = if exit > entry { exit - entry } else { 24 - entry + exit };
                
                if hours > 10 {
                    employee.salary *= 1.1;
                } else {
                    employee.salary *= 0.9;
                }
            }
        }
    }
}