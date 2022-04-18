pub struct ZipFileEntry {
    body: Box<[u8]>,
    crc: u32,
    file_name: String,
}

impl ZipFileEntry {
    fn get_header_size(&self) -> u32 {
        // The file header features a fixed 30 bytes of data plus a variable length file name
        let base_header_size: u32 = 30;
        let file_name_size: u32 = self.file_name.len() as u32;

        base_header_size + file_name_size
    }

    // fn get_header() -> Box<[u8]> {
    //     todo!()
    // }
    //
    // fn get_central_directory_header_size() -> Box<[u8]> {
    //     todo!()
    // }
    //
    // fn get_central_directory_header() -> Box<[u8]> {
    //     todo!()
    // }
    //
    // fn get_body_size() -> u32 {
    //     todo!()
    // }
    //
    // fn get_body() -> Box<[u8]> {
    //     todo!()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_size_with_short_file_name() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("yow")
        };

        let header_size = file_entry.get_header_size();

        assert_eq!(header_size, 33);
    }

    #[test]
    fn header_size_with_long_file_name() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Capoo The BugCat Makes His Move")
        };

        let header_size = file_entry.get_header_size();

        assert_eq!(header_size, 61);
    }

    #[test]
    fn local_file_header_signature() {
        let input_byte: &[u8] = &[0x42];
    }

    #[test]
    fn local_file_header_minimum_version() {
        let input_byte: &[u8] = &[0x42];
    }

    #[test]
    fn local_file_header_general_purpose_bit_flag() {
        let input_byte: &[u8] = &[0x42];
    }
}