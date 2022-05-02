use controls::process_list::ProcessList;
use nwd::NwgUi;
use nwg::NativeUi;
mod controls;

#[derive(Default, NwgUi)]
pub struct TreeViewApp {
    #[nwg_control(size: (800, 600), title: "Task Manager")]
    #[nwg_events( OnWindowClose: [TreeViewApp::exit], OnInit: [TreeViewApp::refresh_processes])]
    window: nwg::Window,

    #[nwg_partial(parent: window)]
    process_list: ProcessList,
}

impl TreeViewApp {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn refresh_processes(&self) {
        self.process_list.load_data();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = TreeViewApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
