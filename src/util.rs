use std::path::PathBuf;

// Clamp forces a value in between to specified values.
// Returns the value between the given range.
pub fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: std::cmp::PartialOrd,
{
    //assert!(min <= max);
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

// The normalize function takes a value between a range of numbers and normalize
// it between a new range of numbers. For instance, if the range is 10..20 with
// a value of 15, and the new range is 0..100, the new value will be 50.
pub fn normalize<T, I: 'static>(value: T, from_min: I, from_max: I, to_min: I, to_max: I) -> I
where
    T: num::cast::AsPrimitive<I>,
    I: Copy + num::Num,
{
    to_min + ((value.as_() - from_min) * (to_max - to_min)) / (from_max - from_min)
}

// Will create all folders if the folders do not exist
pub fn check_folder_exists(folder: &PathBuf) {
    let cur_dir = std::env::current_dir().unwrap();

    if std::path::Path::exists(folder) {
        std::fs::create_dir_all(cur_dir.join(folder)).unwrap();
    }
}

// Get files will return all the files in a given folder.
pub fn get_files(in_folder: &PathBuf) -> Vec<PathBuf> {
    // If folders dont exist, create them.
    check_folder_exists(in_folder);

    // Get the paths of all the files.
    std::fs::read_dir(&in_folder)
        .unwrap()
        .map(|d| d.unwrap().path())
        .collect::<Vec<PathBuf>>()
}