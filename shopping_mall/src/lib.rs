pub mod mall;

pub use floor::store;
pub use mall::*;
pub use store::employee;

// Find and return the largest store in the mall by square meters
pub fn biggest_store(mall: mall::Mall) -> store::Store {
    let mut largest_store: store::Store = store::Store::new("", 0, vec![]);

    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            if store.square_meters > largest_store.square_meters {
                largest_store = store.clone();
            }
        }
    }

    largest_store
}


pub fn highest_paid_employee(mall: mall::Mall) -> Vec<employee::Employee> {
    let mut highest_paid = vec![employee::Employee::new("", 0, 0, 0, 0.0)];

    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            for employee in store.employees.clone().into_iter() {
                if employee.salary > highest_paid[0].salary {
                    highest_paid[0] = employee.clone();
                } else if employee.salary == highest_paid[0].salary {
                    highest_paid.push(employee.clone());
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    let mut total_employees = 0;

    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            total_employees += store.employees.len();
        }
    }

    total_employees + mall.guards.len()
}


pub fn check_for_securities(mall: &mut mall::Mall, available_guards: Vec<guard::Guard>) {
    let mut total_mall_size = 0;

    for floor in mall.floors.iter() {
        total_mall_size += floor.size_limit;
    }

    let mut guard_index = 0;
    while (mall.guards.len() as f64) < total_mall_size as f64 / 200.0 {
        mall.hire_guard(available_guards[guard_index].clone());
        guard_index += 1;
    }
}


pub fn cut_or_raise(mall: &mut mall::Mall) {
    for (floor_index, floor) in mall.clone().floors.iter().enumerate() {
        for (store_index, store) in floor.stores.iter().enumerate() {
            for (employee_index, employee) in store.employees.iter().enumerate() {
                if employee.working_hours.1 - employee.working_hours.0 >= 10 {
                    mall.floors[floor_index].stores[store_index].employees[employee_index].raise(employee.salary * 0.1);
                } else {
                    mall.floors[floor_index].stores[store_index].employees[employee_index].cut(employee.salary * 0.1);
                }
            }
        }
    }
}