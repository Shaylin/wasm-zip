pub struct ZipFileEntry {
    pub body: Vec<u8>,
    pub crc: u32,
    pub file_name: String,
    pub dos_time: u16,
    pub dos_date: u16,
    pub entry_offset: u32,
}

impl ZipFileEntry {
    pub fn get_local_file_header_size(&self) -> usize {
        let base_header_size = 30;
        let file_name_size = self.file_name.len();

        base_header_size + file_name_size
    }

    pub fn get_local_file_header(&self) -> Vec<u8> {
        let mut local_file_header: Vec<u8> = Vec::with_capacity(self.get_local_file_header_size());

        let mut local_file_archive_details_section = self.get_local_file_archive_details_section();
        let mut modified_time_header_section = self.get_modified_time_header_section();
        let mut crc_and_size_header_section = self.get_crc_and_size_header_section();
        let mut local_file_name_header_section = self.get_local_file_name_header_section();

        local_file_header.append(&mut local_file_archive_details_section);
        local_file_header.append(&mut modified_time_header_section);
        local_file_header.append(&mut crc_and_size_header_section);
        local_file_header.append(&mut local_file_name_header_section);

        local_file_header
    }

    fn get_local_file_archive_details_section(&self) -> Vec<u8> {
        let archive_details_section: Vec<u8> = vec![
            0x50, 0x4B, 0x03, 0x04,     // local file header signature
            0x0A, 0x00,                 // version needed to extract
            0x00, 0x00,                 // general purpose bit flag
            0x00, 0x00,                 // compression method
        ];

        archive_details_section
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

    pub fn get_central_directory_header_size(&self) -> usize {
        let base_header_size = 46;

        base_header_size + self.file_name.len()
    }

    pub fn get_central_directory_header(&self) -> Vec<u8> {
        let mut central_directory_header: Vec<u8> = Vec::with_capacity(self.get_central_directory_header_size());

        let mut central_directory_archive_header_section = self.get_central_directory_archive_details_section();
        let mut modified_time_header_section = self.get_modified_time_header_section();
        let mut crc_and_size_header_section = self.get_crc_and_size_header_section();
        let mut central_directory_details_header_section = self.get_central_directory_details_header_section();

        central_directory_header.append(&mut central_directory_archive_header_section);
        central_directory_header.append(&mut modified_time_header_section);
        central_directory_header.append(&mut crc_and_size_header_section);
        central_directory_header.append(&mut central_directory_details_header_section);

        central_directory_header
    }

    fn get_central_directory_archive_details_section(&self) -> Vec<u8> {
        let archive_details_section: Vec<u8> = vec![
            0x50, 0x4B, 0x01, 0x02,     // central directory header signature
            0x3F, 0x00,                 // version made by
            0x0A, 0x00,                 // version needed to extract
            0x00, 0x00,                 // general purpose bit flag
            0x00, 0x00,                 // compression method
        ];

        archive_details_section
    }

    fn get_central_directory_details_header_section(&self) -> Vec<u8> {
        let mut name_header_section: Vec<u8> = Vec::with_capacity(18 + self.file_name.len());

        let name_length = self.file_name.len() as u16;
        let mut file_name_length = Vec::from(name_length.to_le_bytes());
        let mut extra_field_length = vec![0x00, 0x00];
        let mut file_comment_length = vec![0x00, 0x00];
        let mut disk_number_start = vec![0x00, 0x00];
        let mut internal_file_attributes = vec![0x00, 0x00];
        let mut external_file_attributes = vec![0x00, 0x00, 0x00, 0x00];
        let mut relative_offset = Vec::from(self.entry_offset.to_le_bytes());
        let mut file_name = Vec::from(self.file_name.as_bytes());

        name_header_section.append(&mut file_name_length);
        name_header_section.append(&mut extra_field_length);
        name_header_section.append(&mut file_comment_length);
        name_header_section.append(&mut disk_number_start);
        name_header_section.append(&mut internal_file_attributes);
        name_header_section.append(&mut external_file_attributes);
        name_header_section.append(&mut relative_offset);
        name_header_section.append(&mut file_name);

        name_header_section
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
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
            entry_offset: 0,
        };

        let local_file_header = file_entry.get_local_file_header();

        assert_eq!(given_file_name.into_bytes(), &local_file_header[30..local_file_header.len()]);
    }

    #[test]
    fn central_directory_header_size_with_small_file_name() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("yo"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let header_size = file_entry.get_central_directory_header_size();

        assert_eq!(header_size, 48);
    }

    #[test]
    fn central_directory_header_size_with_large_file_name() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("This Is An Exceedingly Long File Name With Many Characters.txt"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let header_size = file_entry.get_central_directory_header_size();

        assert_eq!(header_size, 108);
    }

    #[test]
    fn central_directory_header_signature() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("This Is An Exceedingly Long File Name With Many Characters.txt"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x50, 0x4B, 0x01, 0x02], &central_directory_header[0..4]);
    }

    #[test]
    fn central_directory_header_version_made_by() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Ba"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x3F, 0x00], &central_directory_header[4..6]);
    }

    #[test]
    fn central_directory_header_version_needed_to_extract() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Ba"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x0A, 0x00], &central_directory_header[6..8]);
    }

    #[test]
    fn central_directory_header_general_purpose_bit_flag() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Ba"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x00, 0x00], &central_directory_header[8..10]);
    }

    #[test]
    fn central_directory_header_compression_method() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Ba"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x00, 0x00], &central_directory_header[10..12]);
    }

    #[test]
    fn central_directory_header_modified_time() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Ba"),
            dos_time: 0xFF84,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x84, 0xFF], &central_directory_header[12..14]);
    }

    #[test]
    fn central_directory_header_modified_date() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Ba"),
            dos_time: 0,
            dos_date: 0xFEEB,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0xEB, 0xFE], &central_directory_header[14..16]);
    }

    #[test]
    fn central_directory_header_crc() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0xBEAD1234,
            file_name: String::from("Ba"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x34, 0x12, 0xAD, 0xBE], &central_directory_header[16..20]);
    }

    #[test]
    fn central_directory_header_compressed_size() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Ba"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x05, 0x00, 0x00, 0x00], &central_directory_header[20..24]);
    }

    #[test]
    fn central_directory_header_uncompressed_size() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3, 4, 5]),
            crc: 0,
            file_name: String::from("Foam Cat"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x0D, 0x00, 0x00, 0x00], &central_directory_header[24..28]);
    }

    #[test]
    fn central_directory_header_file_name_length() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3, 4, 5]),
            crc: 0,
            file_name: String::from("Foam Cat"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x08, 0x00], &central_directory_header[28..30]);
    }

    #[test]
    fn central_directory_header_extra_field_length() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3, 4, 5]),
            crc: 0,
            file_name: String::from("Foam Cat"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x00, 0x00], &central_directory_header[30..32]);
    }

    #[test]
    fn central_directory_header_file_comment_length() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Foam"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x00, 0x00], &central_directory_header[32..34]);
    }

    #[test]
    fn central_directory_header_disk_number_start() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Foam"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x00, 0x00], &central_directory_header[34..36]);
    }

    #[test]
    fn central_directory_header_internal_file_attributes() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Foam"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x00, 0x00], &central_directory_header[36..38]);
    }

    #[test]
    fn central_directory_header_external_file_attributes() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Foam"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x00, 0x00, 0x00, 0x00], &central_directory_header[38..42]);
    }

    #[test]
    fn central_directory_header_relative_offset() {
        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: String::from("Foam"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0x7712AB32,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!([0x32, 0xAB, 0x12, 0x77], &central_directory_header[42..46]);
    }

    #[test]
    fn central_directory_header_file_name() {
        let given_name = String::from("Foam Cat Takes Over the World");

        let file_entry = ZipFileEntry {
            body: Box::from([1, 2, 3]),
            crc: 0,
            file_name: given_name.clone(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0x7712AB32,
        };

        let central_directory_header = file_entry.get_central_directory_header();

        assert_eq!(given_name.into_bytes(), &central_directory_header[46..central_directory_header.len()]);
    }
}