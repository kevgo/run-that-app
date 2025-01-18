/// the different ways to execute an application
pub enum Method {
  /// execute this app's default executable
  ThisApp {
    /// defines the ways in which this app can be installed
    install_methods: Vec<installation::Method>,
  },
  /// executes another executable (not the default executable) of another app
  OtherAppOtherExecutable {
    /// the other application that contains the executable
    app: Box<dyn App>,
    /// name of the executable to run
    executable_name: String,
  },
  //   /// executes the default executable of another app with additional arguments
  //   OtherAppDefaultExecutable { app: Box<dyn App>, args: Vec<String> },
}
