// use std::os::windows; 

use slint::{FilterModel, Model, SortModel};
use std::rc::Rc;


slint::include_modules!();

pub fn main() {
    
    let todo_model = Rc::new(slint::VecModel::<TodoItem>::from(vec![
        TodoItem { status: 1, title: "Get Calender integration for how many days an items been pending".into() },
        TodoItem { status: 0, title: "Get an icon, and a application title".into()},
        TodoItem { status: 2, title: "Release it".into()},
        TodoItem { status: 0, title: "Add text wrap".into()}
    ]));
    
    let main_window = MainWindow::new().unwrap();

    main_window.on_todo_added({
        let todo_model = todo_model.clone();
        move |text| todo_model.push(TodoItem { status: 0, title: text})
    });
    
    main_window.on_remove_done({
        let todo_model = todo_model.clone();
        move || {
            let mut offset = 0;
            for i in 0..todo_model.row_count() {
                if todo_model.row_data(i - offset).unwrap().status == 0 {
                    offset += 1;
                }
                else if todo_model.row_data(i-offset).unwrap().status == 1 {
                    offset += 1;
                }
                else if todo_model.row_data(i-offset).unwrap().status == 2 {
                    todo_model.remove(i - offset);
                    offset += 1;
                }
                else {
                    println!("No status");
                }
            }
        }
    });

    // completed to doing crashes
    // doing to pending crashes
    // pending to doing crashes

    let weak_window = main_window.as_weak();
    main_window.on_popup_confirmed(move || {
        let window = weak_window.unwrap();
        window.hide().unwrap();
    });

    {
        let weak_window = main_window.as_weak();
        let todo_model = todo_model.clone();
        main_window.window().on_close_requested(move || {
            let window = weak_window.unwrap();

            if todo_model.iter().any(|e| e.status != 2) {
                window.invoke_show_confirm_popup();
                slint::CloseRequestResponse::KeepWindowShown
            } else {
                slint::CloseRequestResponse::HideWindow
            }
           
           
           
            // let status_int = todo_item.status;
            // if status_int == 0 || status_int == 1 {
            //     window.invoke_show_confirm_popup();
            //     slint::CloseRequestResponse::KeepWindowShown
            // } else {
            //     slint::CloseRequestResponse::HideWindow
            // }
        });
    }

    main_window.on_apply_sorting_and_filtering({
        let weak_window = main_window.as_weak();
        let todo_model = todo_model.clone();
    
        move || {
            let window = weak_window.unwrap();
            window.set_todo_model(todo_model.clone().into());
    
            if window.get_hide_done_items() {
                window.set_todo_model(
                    Rc::new(FilterModel::new(window.get_todo_model(), |e| e.status != 2)).into(),
                );
            }
    
            // Customize the sorting logic based on status
            window.set_todo_model(
                Rc::new(SortModel::new(window.get_todo_model(), |lhs, rhs| {
                    // Sort by status: Pending (0) on top, Doing (1) in the middle, Complete (2) at the bottom
                    lhs.status.cmp(&rhs.status)
                }))
                .into(),
            );
        }
    });

    main_window.set_show_header(true);
    main_window.set_todo_model(todo_model.into());

    main_window.run().unwrap();

}
