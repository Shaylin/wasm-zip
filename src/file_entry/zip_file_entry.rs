pub struct ZipFileEntry {
    body: Box<[u8]>,
    crc: u32,
    file_name: String,
}

impl ZipFileEntry {
    fn get_header_size(&self) -> usize {
        let base_header_size = 30;
        let file_name_size = self.file_name.len();

        base_header_size + file_name_size
    }

    fn get_header(&self) -> Box<[u8]> {
        let mut header_data: Vec<u8> = Vec::with_capacity(self.get_header_size());

        let _local_file_header_signature: Vec<u8> = vec![0x50, 0x4B, 0x03, 0x04];
        let _version_needed_to_extract: Vec<u8> = vec![0x0A, 0x00];
        let _general_purpose_bit_flag: Vec<u8> = vec![0x00, 0x00];
        let _compression_method: Vec<u8> = vec![0x00, 0x00];

        let _last_modified_time:Vec<u8> = vec![0x00, 0x00];
        let _last_modified_date:Vec<u8> = vec![0x00, 0x00];

        let _crc:Vec<u8> = Vec::from(self.crc.to_le_bytes());


        header_data.into_boxed_slice()
    }

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
            file_name: String::from("yow"),
        };

        let header_size = file_entry.get_header_size();

        assert_eq!(header_size, 33);
    }

    #[test]
    fn header_size_with_long_file_name() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Capoo The BugCat Makes His Move"),
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