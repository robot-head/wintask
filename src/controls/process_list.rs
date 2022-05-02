use nwd::NwgPartial;
use nwg::InsertListViewColumn;
use sysinfo::{ProcessExt, ProcessRefreshKind, System, SystemExt};

#[derive(Default, NwgPartial)]
pub struct ProcessList {
    #[nwg_control(list_style: nwg::ListViewStyle::Detailed, focus: true, size: (800, 600),
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT,
    )]
    data_view: nwg::ListView,
}

impl ProcessList {
    pub fn load_data(&self) {
        self.data_view.insert_column(InsertListViewColumn {
            text: Some("Name".to_string()),
            index: Some(0),
            fmt: None,
            width: Some(600),
        });
        self.data_view.insert_column(InsertListViewColumn {
            text: Some("PID".to_string()),
            index: Some(1),
            fmt: None,
            width: Some(100),
        });
        self.data_view.insert_column(InsertListViewColumn {
            text: Some("CPU".to_string()),
            index: Some(2),
            fmt: None,
            width: Some(100),
        });
        self.data_view.set_headers_enabled(true);
        let mut sys = System::default();
        sys.refresh_processes_specifics(ProcessRefreshKind::new().with_cpu());
        for (pid, process) in sys.processes() {
            self.data_view.insert_items_row(
                None,
                &[
                    process.name(),
                    &format!("{}", pid),
                    &format!("{:.2}", process.cpu_usage()),
                ],
            );
        }
    }
}
//
