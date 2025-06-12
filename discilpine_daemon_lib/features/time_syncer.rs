pub struct TimeSyncer {

}

// pub fn sync_time() -> Result<(), ()> {
//   match Command::new("ntpdate").arg("-s").arg("time.nist.gov").output() {
//     Ok(_) => Ok(()),
//     Err(_) => Err(())
//   }
// }