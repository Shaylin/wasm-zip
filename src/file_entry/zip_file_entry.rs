pub struct ZipFileEntry {
    body: Box<[u8]>,
    crc: u32,
    file_name: String,
    dos_time: u16,
    dos_date: u16,
}

impl ZipFileEntry {
    pub fn get_local_file_header_size(&self) -> usize {
        let base_header_size = 30;
        let file_name_size = self.file_name.len();

        base_header_size + file_name_size
    }

    pub fn get_local_file_header(&self) -> Vec<u8> {
        let mut local_file_header: Vec<u8> = Vec::with_capacity(self.get_local_file_header_size());

        let mut archive_details_header_section = self.get_archive_details_header_section();
        let mut modified_time_header_section = self.get_modified_time_header_section();
        let mut crc_and_size_header_section = self.get_crc_and_size_header_section();
        let mut local_file_name_header_section = self.get_local_file_name_header_section();

        local_file_header.append(&mut archive_details_header_section);
        local_file_header.append(&mut modified_time_header_section);
        local_file_header.append(&mut crc_and_size_header_section);
        local_file_header.append(&mut local_file_name_header_section);

        local_file_header
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

    fn get_crc_and_size_header_section(&self) -> Vec<u8> {
        let mut crc_and_size_header_section: Vec<u8> = Vec::with_capacity(12);

        let mut crc = Vec::from(self.crc.to_le_bytes());
        let mut compressed_size = Vec::from(self.get_total_file_size().to_le_bytes());
        let mut uncompressed_size = Vec::from(self.get_total_file_size().to_le_bytes());

        crc_and_size_header_section.append(&mut crc);
        crc_and_size_header_section.append(&mut compressed_size);
        crc_and_size_header_section.append(&mut uncompressed_size);

        crc_and_size_header_section
    }

    fn get_total_file_size(&self) -> u32 {
        let file_name_size: u32 = self.file_name.len() as u32;
        let body_size: u32 = self.body.len() as u32;

        file_name_size + body_size
    }

    fn get_local_file_name_header_section(&self) -> Vec<u8> {
        let mut local_file_name_header_section: Vec<u8> = Vec::with_capacity(4 + self.file_name.len());

        let name_length = self.file_name.len() as u16;
        let mut file_name_length = Vec::from(name_length.to_le_bytes());

        let mut extra_field_length = vec![0x00, 0x00];

        let mut file_name = Vec::from(self.file_name.as_bytes());

        local_file_name_header_section.append(&mut file_name_length);
        local_file_name_header_section.append(&mut extra_field_length);
        local_file_name_header_section.append(&mut file_name);

        local_file_name_header_section
    }

    fn get_central_directory_header_size(&self) -> usize {
        let base_header_size = 46;

        base_header_size + self.file_name.len()
    }

    fn get_central_directory_header(&self) -> Vec<u8> {
        let mut central_directory_header: Vec<u8> = Vec::with_capacity(self.get_central_directory_header_size());

        central_directory_header
    }
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

        let header_size = file_entry.get_local_file_header_size();

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

        let header_size = file_entry.get_local_file_header_size();

        assert_eq!(header_size, 61);
    }

    #[test]
    fn local_file_header_signature() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Capoo"),
            dos_time: 0,
            dos_date: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0x50, 0x4B, 0x03, 0x04], &local_file_header[0..4]);
    }

    #[test]
    fn local_file_header_minimum_version() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Capoo"),
            dos_time: 0,
            dos_date: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0x0A, 0x00], &local_file_header[4..6]);
    }

    #[test]
    fn local_file_header_general_purpose_bit_flag() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Capoo"),
            dos_time: 0,
            dos_date: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0x00, 0x00], &local_file_header[6..8]);
    }

    #[test]
    fn local_file_header_compression_method() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Capoo"),
            dos_time: 0,
            dos_date: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0x00, 0x00], &local_file_header[8..10]);
    }

    #[test]
    fn local_file_header_modified_time() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Capoo"),
            dos_time: 0x5611,
            dos_date: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0x11, 0x56], &local_file_header[10..12]);
    }

    #[test]
    fn local_file_header_modified_date() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Capoo"),
            dos_time: 0,
            dos_date: 0x88AC,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0xAC, 0x88], &local_file_header[12..14]);
    }

    #[test]
    fn local_file_header_crc() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0x11223344,
            file_name: String::from("Capoo"),
            dos_time: 0,
            dos_date: 0x88AC,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0x44, 0x33, 0x22, 0x11], &local_file_header[14..18]);
    }

    #[test]
    fn local_file_header_compressed_size() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Capoo"),
            dos_time: 0,
            dos_date: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0x08, 0x00, 0x00, 0x00], &local_file_header[18..22]);
    }

    #[test]
    fn local_file_header_uncompressed_size() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3, 4]),
            crc: 0,
            file_name: String::from("Capoo The BugCat"),
            dos_time: 0,
            dos_date: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0x14, 0x00, 0x00, 0x00], &local_file_header[22..26]);
    }

    #[test]
    fn local_file_header_file_name_length() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3, 4]),
            crc: 0,
            file_name: String::from("FoamCat/CafeIsGood.txt"),
            dos_time: 0,
            dos_date: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0x16, 0x00], &local_file_header[26..28]);
    }

    #[test]
    fn local_file_header_extra_field_length() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("FoamCat"),
            dos_time: 0,
            dos_date: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!([0x00, 0x00], &local_file_header[28..30]);
    }

    #[test]
    fn local_file_header_file_name() {
        let given_file_name = String::from("This is a test file name.png");

        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: given_file_name.clone(),
            dos_time: 0,
            dos_date: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!(given_file_name.into_bytes(), &local_file_header[30..local_file_header.len()]);
    }
}