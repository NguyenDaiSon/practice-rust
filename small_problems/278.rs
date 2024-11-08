// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut bad_version, mut first_version, mut last_version) = (-1, 1, n);

        while first_version <= last_version {
            let version = first_version + (last_version - first_version) / 2;

            if self.isBadVersion(version) {
                bad_version = version;
                last_version = version - 1;
            } else {
                first_version = version + 1;
            }
        }

        bad_version
    }
}
