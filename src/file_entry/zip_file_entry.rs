pub struct ZipFileEntry {
    body: Box<[u8]>,
    crc: u32,
    file_name: String,
    dos_time: u16,
    dos_date: u16,
}

impl ZipFileEntry {
    pub fn get_header_size(&self) -> usize {
        let base_header_size = 30;
        let file_name_size = self.file_name.len();

        base_header_size + file_name_size
    }

    pub fn get_header(&self) -> Box<[u8]> {
        let mut header_data: Vec<u8> = Vec::with_capacity(self.get_header_size());

        let mut archive_details_header_section = self.get_archive_details_header_section();
        let mut modified_time_header_section = self.get_modified_time_header_section();
        let mut get_file_details_header_section = self.get_file_details_header_section();

        header_data.append(&mut archive_details_header_section);
        header_data.append(&mut modified_time_header_section);
        header_data.append(&mut get_file_details_header_section);

        header_data.into_boxed_slice()
    }

    fn get_total_file_size(&self) -> u32 {
        let file_name_size: u32 = self.file_name.len() as u32;
        let body_size: u32 = self.body.len() as u32;

        file_name_size + body_size
    }

    fn get_archive_details_header_section(&self) -> Vec<u8> {
        let metadata_header_section: Vec<u8> = vec![
            0x50, 0x4B, 0x03, 0x04,     // local file header signature
            0x0A, 0x00,                 // version needed to extract
            0x00, 0x00,                 // general purpose bit flag
            0x00, 0x00,                 // compression method
        ];

        metadata_header_section
    }

    fn get_modified_time_header_section(&self) -> Vec<u8> {
        let mut last_modified_time = Vec::from(self.dos_time.to_le_bytes());
        let mut last_modified_date = Vec::from(self.dos_date.to_le_bytes());

        last_modified_time.append(&mut last_modified_date);

        last_modified_time
    }

    fn get_file_details_header_section(&self) -> Vec<u8> {
        let mut file_details_header_section: Vec<u8> = Vec::new();

        let mut crc = Vec::from(self.crc.to_le_bytes());

        let mut compressed_size = Vec::from(self.get_total_file_size().to_le_bytes());
        let mut uncompressed_size = Vec::from(self.get_total_file_size().to_le_bytes());

        let name_length: u16 = self.file_name.len() as u16;
        let mut file_name_length = Vec::from(name_length.to_le_bytes());

        let mut extra_field_length = vec![0x00, 0x00];

        let mut file_name = Vec::from(self.file_name.as_bytes());

        file_details_header_section.append(&mut crc);
        file_details_header_section.append(&mut compressed_size);
        file_details_header_section.append(&mut uncompressed_size);
        file_details_header_section.append(&mut file_name_length);
        file_details_header_section.append(&mut extra_field_length);
        file_details_header_section.append(&mut file_name);

        file_details_header_section
    }

    // fn get_central_directory_header_size() -> Box<[u8]> {
    //     todo!()
    // }
    //
    // fn get_central_directory_header() -> Box<[u8]> {
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
            dos_time: 0,
            dos_date: 0,
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
            dos_time: 0,
            dos_date: 0,
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