use std::time::Duration;

/// Formats the time from seconds to a string with the format yy:ww:dd:hh:mm:ss
/// depending on how long the system has been running
/// ### Parameters
/// * `reftime` - The reference to the time in seconds
pub fn parse_time(reftime: &Duration) -> String {
  let time: u64 = reftime.as_secs();
  let mut time_str: String = String::new();
  let mut time_vec: Vec<u64> = vec![];
  let unit_vec: Vec<String> = vec![
      String::from("y"),
      String::from("w"),
      String::from("d"),
      String::from("h"),
      String::from("m"),
      String::from("s"),
  ];

  time_vec.push(time / 31536000);
  time_vec.push((time / 604800) % 52);
  time_vec.push((time / 86400) % 7);
  time_vec.push((time / 3600) % 24);
  time_vec.push((time / 60) % 60);
  time_vec.push(time % 60);

  for i in 0..time_vec.len() {
      if time_vec[i] != 0 {
          time_str += &format!("{}{} ", time_vec[i], unit_vec[i]);
      }
  }
  return time_str;
}

/// Parses the given size into string with right size suffix and returns it
/// ### Parameters
/// * `refsize` - The reference to the size to be parsed
pub fn parse_size(refsize: &u64) -> String {
  let mut size: f32 = *refsize as f32;
  let mut unit_index: usize = 0;
  let unit_vec: Vec<String> = vec![
      String::from("B"),
      String::from("KB"),
      String::from("MB"),
      String::from("GB"),
      String::from("TB"),
      String::from("PB"),
      String::from("EB"),
      String::from("ZB"),
      String::from("YB"),
  ];
  while size > 1024.0 {
      size /= 1024.0;
      unit_index += 1;
  }
  return format!("{:.2}{}", size, unit_vec[unit_index]);
}