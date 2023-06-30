use std::fs;
use std::path::{Path, PathBuf};

/// Returns a list of devices currently known to the system
///
/// # Example
///
/// ```
/// use v4l::context;
/// for dev in context::enum_devices() {
///     print!("{}{}", dev.index(), dev.name().unwrap());
/// }
/// ```
pub fn enum_devices() -> Vec<Node> {
    let mut devices = Vec::new();

    let entries = fs::read_dir("/dev");
    if let Ok(entries) = entries {
        for dentry in entries {
            let dentry = match dentry {
                Ok(dentry) => dentry,
                Err(_) => continue,
            };

            let file_name = dentry.file_name();
            let file_name = file_name.to_str().unwrap();

            if file_name.starts_with("video") || file_name.starts_with("v4l-subdev") {
                let node = Node::new(dentry.path());
                devices.push(node);
            }
        }
    }

    devices
}

/// Represents a video4linux device node
pub struct Node {
    /// Device node path
    path: PathBuf,
}

impl Node {
    /// Returns a device node observer by path
    ///
    /// The device is opened in read only mode.
    ///
    /// # Arguments
    ///
    /// * `path` - Node path (usually a character device or sysfs entry)
    ///
    /// # Example
    ///
    /// ```
    /// use v4l::context::Node;
    /// let node = Node::new("/dev/video0");
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Node {
            path: PathBuf::from(path.as_ref()),
        }
    }

    /// Returns the absolute path of the device node
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the index of the device node
    pub fn index(&self) -> usize {
        let file_name = self.path.file_name().unwrap();

        let mut index_str = String::new();
        let file_name_str = file_name.to_str().unwrap();
        let file_name_str = if self.is_v4l_subdev() {
            file_name_str.strip_prefix("v4l-subdev").unwrap()
        } else {
            file_name_str.strip_prefix("video").unwrap()
        };
        for c in file_name_str.chars() {
            if !c.is_ascii_digit() {
                continue;
            }

            index_str.push(c);
        }

        let index = index_str.parse::<usize>();
        index.unwrap()
    }

    /// Returns name of the device by parsing its sysfs entry
    pub fn name(&self) -> Option<String> {
        let index = self.index();
        let root = if self.is_v4l_subdev() {
            "/sys/class/video4linux/v4l-subdev"
        } else {
            "/sys/class/video4linux/video"
        };
        let path = format!("{}{}{}", root, index, "/name");
        let name = fs::read_to_string(path);
        match name {
            Ok(name) => Some(name.trim().to_string()),
            Err(_) => None,
        }
    }

    /// Returns true if the device is a v4l subdev
    pub fn is_v4l_subdev(&self) -> bool {
        self.path
            .file_name()
            .map(|x| {
                x.to_str()
                    .map(|y| y.starts_with("v4l-subdev"))
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }
}
