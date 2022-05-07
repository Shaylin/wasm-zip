use std::collections::HashMap;

use crate::CrcCalculator;
use crate::date_time_converter::DosDateTimeCalculator;
use crate::zip_file::zip_file_entry::ZipFileEntry;
use crate::zip_file::ZipBlobFactory;

pub struct ZipBlobFactoryAdapter {
    pub(crate) crc_calculator: Box<dyn CrcCalculator>,
    pub(crate) date_time_retriever: Box<dyn DosDateTimeCalculator>,
}

impl ZipBlobFactory for ZipBlobFactoryAdapter {
    fn create_zip_binary(&self, directory_mapping: HashMap<String, Vec<u8>>) -> Box<[u8]> {
        let zip_file_entries = self.create_zip_file_entries(directory_mapping);

        let mut zip_blob: Vec<u8> = Vec::new();
        let mut central_directory_records: Vec<u8> = Vec::new();
        let mut end_of_central_directory_record = self.get_end_of_central_directory_record(&zip_file_entries);

        for file_entry in zip_file_entries {
            let mut local_file_header = file_entry.get_local_file_header();
            let mut central_directory_header = file_entry.get_central_directory_header();
            let mut file_body = file_entry.body;
            zip_blob.append(&mut local_file_header);
            zip_blob.append(&mut file_body);
            central_directory_records.append(&mut central_directory_header);
        }

        zip_blob.append(&mut central_directory_records);
        zip_blob.append(&mut end_of_central_directory_record);

        zip_blob.into_boxed_slice()
    }
}

impl ZipBlobFactoryAdapter {
    fn create_zip_file_entries(&self, directory_mapping: HashMap<String, Vec<u8>>) -> Vec<ZipFileEntry> {
        let mut zip_file_entries: Vec<ZipFileEntry> = Vec::new();

        let mut file_header_offset: u32 = 0;

        for (file_name, file_body) in directory_mapping.into_iter() {
            let zip_entry = self.create_zip_file_entry(file_name, file_body, file_header_offset);
            file_header_offset += self.get_zip_file_size(&zip_entry);
            zip_file_entries.push(zip_entry);
        }

        zip_file_entries
    }

    fn create_zip_file_entry(&self, zip_file_name: String, file_body: Vec<u8>, header_offset: u32) -> ZipFileEntry {
        let file_crc = self.calculate_file_crc(&file_body);

        ZipFileEntry {
            body: file_body,
            crc: file_crc,
            file_name: zip_file_name,
            dos_time: self.date_time_retriever.get_current_dos_time(),
            dos_date: self.date_time_retriever.get_current_dos_date(),
            entry_offset: header_offset,
        }
    }

    fn calculate_file_crc(&self, file_contents: &[u8]) -> u32 {
        self.crc_calculator.calculate_crc32(file_contents)
    }

    fn get_zip_file_size(&self, zip_file: &ZipFileEntry) -> u32 {
        let local_file_header_size = zip_file.get_local_file_header_size() as u32;
        let body_size = zip_file.body.len() as u32;

        local_file_header_size + body_size
    }

    fn get_end_of_central_directory_record(&self, zip_file_entries: &[ZipFileEntry]) -> Vec<u8> {
        let mut end_of_central_directory_record: Vec<u8> = Vec::with_capacity(22);

        let mut disk_info_section: Vec<u8> = vec![
            0x50, 0x4B, 0x05, 0x06,     //end of central directory signature
            0x00, 0x00,                 //number of this disk
            0x00, 0x00,                 //disk where central directory starts
        ];

        let mut total_number_of_central_directory_records = self.get_number_of_central_directory_records_section(zip_file_entries);
        let mut number_of_central_directory_records_on_disk = self.get_number_of_central_directory_records_section(zip_file_entries);
        let mut size_of_central_directory_section = self.get_size_of_central_directory_section(zip_file_entries);
        let mut central_directory_start_section = self.get_central_directory_start_offset_section(zip_file_entries);

        let mut comment_length: Vec<u8> = vec![0x00, 0x00];

        end_of_central_directory_record.append(&mut disk_info_section);
        end_of_central_directory_record.append(&mut total_number_of_central_directory_records);
        end_of_central_directory_record.append(&mut number_of_central_directory_records_on_disk);
        end_of_central_directory_record.append(&mut size_of_central_directory_section);
        end_of_central_directory_record.append(&mut central_directory_start_section);
        end_of_central_directory_record.append(&mut comment_length);

        end_of_central_directory_record
    }

    fn get_number_of_central_directory_records_section(&self, zip_file_entries: &[ZipFileEntry]) -> Vec<u8> {
        let number_of_central_directory_records = zip_file_entries.len() as u16;

        Vec::from(number_of_central_directory_records.to_le_bytes())
    }

    fn get_size_of_central_directory_section(&self, zip_file_entries: &[ZipFileEntry]) -> Vec<u8> {
        let mut central_directory_size: u32 = 0;

        for zip_file_entry in zip_file_entries {
            central_directory_size += zip_file_entry.get_central_directory_header_size() as u32;
        }

        Vec::from(central_directory_size.to_le_bytes())
    }

    fn get_central_directory_start_offset_section(&self, zip_file_entries: &[ZipFileEntry]) -> Vec<u8> {
        let mut start_offset: u32 = 0;

        for zip_file_entry in zip_file_entries {
            start_offset += zip_file_entry.get_local_file_header_size() as u32;
            start_offset += zip_file_entry.body.len() as u32;
        }

        Vec::from(start_offset.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeCrcCalculator {}

    impl CrcCalculator for FakeCrcCalculator {
        fn calculate_crc32(&self, _data: &[u8]) -> u32 {
            0x11223344
        }
    }

    struct FakeDosDateTimeRetriever {}

    impl DosDateTimeCalculator for FakeDosDateTimeRetriever {
        fn get_current_dos_time(&self) -> u16 {
            0x1133
        }

        fn get_current_dos_date(&self) -> u16 {
            0x9988
        }
    }

    #[test]
    fn zip_file_size_should_be_the_sum_of_body_header_lengths() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let fake_file_entry = ZipFileEntry {
            body: vec![0; 293],
            crc: 0,
            file_name: String::from("blab"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        };

        assert_eq!(327, blob_factory_adapter.get_zip_file_size(&fake_file_entry));
    }

    #[test]
    fn calculate_file_crc_should_delegate_to_crc_calculator() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let fake_file_body: Vec<u8> = vec![0; 33];

        assert_eq!(0x11223344, blob_factory_adapter.calculate_file_crc(&fake_file_body));
    }

    #[test]
    fn create_zip_file_entry_should_create_zip_entry_with_given_data() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let file_name = String::from("BugCat");
        let file_body: Vec<u8> = vec![0; 33];
        let header_offset: u32 = 98;

        let created_file_entry = blob_factory_adapter.create_zip_file_entry(file_name.clone(), file_body.clone(), header_offset);

        assert_eq!(file_name, created_file_entry.file_name);
        assert_eq!(file_body, created_file_entry.body);
        assert_eq!(header_offset, created_file_entry.entry_offset);
        assert_eq!(0x11223344, created_file_entry.crc);
        assert_eq!(0x1133, created_file_entry.dos_time);
        assert_eq!(0x9988, created_file_entry.dos_date);
    }

    #[test]
    fn creating_multiple_entries_from_directory_hash_map_with_sequential_header_offsets() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let given_hash_map: HashMap<String, Vec<u8>> = HashMap::from([
            (String::from("BugCat.txt"), vec![2; 16]),
            (String::from("MyFolder/FoamCat.txt"), vec![0; 11])
        ]);

        let created_file_entries = blob_factory_adapter.create_zip_file_entries(given_hash_map);

        //The consuming iterator used to create the vector has arbitrary ordering
        if created_file_entries[0].file_name == String::from("BugCat.txt") {
            assert_eq!(vec![2; 16], created_file_entries[0].body);
            assert_eq!(0, created_file_entries[0].entry_offset);

            assert_eq!(vec![0; 11], created_file_entries[1].body);
            assert_eq!(56, created_file_entries[1].entry_offset);
        } else {
            assert_eq!(vec![0; 11], created_file_entries[0].body);
            assert_eq!(0, created_file_entries[0].entry_offset);

            assert_eq!(vec![2; 16], created_file_entries[1].body);
            assert_eq!(61, created_file_entries[1].entry_offset);
        }
    }

    #[test]
    fn number_of_central_directory_records_with_one_entry() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![],
            crc: 0,
            file_name: "a".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });

        let number_of_central_directory_records = blob_factory_adapter.get_number_of_central_directory_records_section(&zip_entries);

        assert_eq!([0x01, 0x00], &number_of_central_directory_records[0..2]);
    }

    #[test]
    fn number_of_central_directory_records_with_many_entries() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        for _entry_number in 0..1000 {
            zip_entries.push(ZipFileEntry {
                body: vec![],
                crc: 0,
                file_name: "a".to_string(),
                dos_time: 0,
                dos_date: 0,
                entry_offset: 0,
            });
        }

        let number_of_central_directory_records = blob_factory_adapter.get_number_of_central_directory_records_section(&zip_entries);

        assert_eq!([0xE8, 0x03], &number_of_central_directory_records[0..2]);
    }

    #[test]
    fn central_directory_size_with_one_entry() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![0; 10],
            crc: 0,
            file_name: "BugCat.txt".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });

        let central_directory_size = blob_factory_adapter.get_size_of_central_directory_section(&zip_entries);

        assert_eq!([0x38, 0x00, 0x00, 0x00], &central_directory_size[0..4]);
    }

    #[test]
    fn central_directory_size_with_many_entries() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        for _entry_number in 0..10 {
            zip_entries.push(ZipFileEntry {
                body: vec![0; 10],
                crc: 0,
                file_name: "BugCat.txt".to_string(),
                dos_time: 0,
                dos_date: 0,
                entry_offset: 0,
            });
        }

        let central_directory_size = blob_factory_adapter.get_size_of_central_directory_section(&zip_entries);

        assert_eq!([0x30, 0x02, 0x00, 0x00], &central_directory_size[0..4]);
    }

    #[test]
    fn central_directory_offset_with_one_entry() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![0; 10],
            crc: 0,
            file_name: "BugCat.txt".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });

        let central_directory_offset = blob_factory_adapter.get_central_directory_start_offset_section(&zip_entries);

        assert_eq!([0x32, 0x00, 0x00, 0x00], &central_directory_offset[0..4]);
    }

    #[test]
    fn end_of_central_directory_signature() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![0; 10],
            crc: 0,
            file_name: "BugCat.txt".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });


        let end_of_central_directory = blob_factory_adapter.get_end_of_central_directory_record(&zip_entries);

        assert_eq!([0x50, 0x4B, 0x05, 0x06], &end_of_central_directory[0..4]);
    }

    #[test]
    fn end_of_central_directory_number_of_disk() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![0; 10],
            crc: 0,
            file_name: "BugCat.txt".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });


        let end_of_central_directory = blob_factory_adapter.get_end_of_central_directory_record(&zip_entries);

        assert_eq!([0x00, 0x00], &end_of_central_directory[4..6]);
    }

    #[test]
    fn end_of_central_directory_disk_where_central_directory_starts() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![0; 10],
            crc: 0,
            file_name: "BugCat.txt".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });


        let end_of_central_directory = blob_factory_adapter.get_end_of_central_directory_record(&zip_entries);

        assert_eq!([0x00, 0x00], &end_of_central_directory[6..8]);
    }

    #[test]
    fn end_of_central_directory_number_of_records_on_disk() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![0; 10],
            crc: 0,
            file_name: "BugCat.txt".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });


        let end_of_central_directory = blob_factory_adapter.get_end_of_central_directory_record(&zip_entries);

        assert_eq!([0x01, 0x00], &end_of_central_directory[8..10]);
    }

    #[test]
    fn end_of_central_directory_total_number_of_records() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![0; 10],
            crc: 0,
            file_name: "BugCat.txt".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });


        let end_of_central_directory = blob_factory_adapter.get_end_of_central_directory_record(&zip_entries);

        assert_eq!([0x01, 0x00], &end_of_central_directory[10..12]);
    }

    #[test]
    fn end_of_central_directory_size_of_central_directory() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![0; 10],
            crc: 0,
            file_name: "BugCat.txt".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });


        let end_of_central_directory = blob_factory_adapter.get_end_of_central_directory_record(&zip_entries);

        assert_eq!([0x38, 0x00, 0x00, 0x00], &end_of_central_directory[12..16]);
    }

    #[test]
    fn end_of_central_directory_start_offset_of_central_directory() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![0; 10],
            crc: 0,
            file_name: "BugCat.txt".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });


        let end_of_central_directory = blob_factory_adapter.get_end_of_central_directory_record(&zip_entries);

        assert_eq!([0x32, 0x00, 0x00, 0x00], &end_of_central_directory[16..20]);
    }

    #[test]
    fn end_of_central_directory_start_comment_length() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let mut zip_entries: Vec<ZipFileEntry> = Vec::new();

        zip_entries.push(ZipFileEntry {
            body: vec![0; 10],
            crc: 0,
            file_name: "BugCat.txt".to_string(),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0,
        });


        let end_of_central_directory = blob_factory_adapter.get_end_of_central_directory_record(&zip_entries);

        assert_eq!([0x00, 0x00], &end_of_central_directory[20..22]);
    }

    #[test]
    fn single_file_zip_blob_header_signatures() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let file_contents = String::from("Capoo is Hungry.");

        let input_map: HashMap<String, Vec<u8>> = HashMap::from([
            (String::from("Hello.txt"), Vec::from(file_contents.as_bytes())),
        ]);

        let zip_blob = blob_factory_adapter.create_zip_binary(input_map);

        assert_eq!(132, zip_blob.len());

        assert_eq!([0x50, 0x4B, 0x03, 0x04], &zip_blob[0..4]);
        assert_eq!([0x50, 0x4B, 0x01, 0x02], &zip_blob[55..59]);
        assert_eq!([0x50, 0x4B, 0x05, 0x06], &zip_blob[110..114]);
    }

    #[test]
    fn multiple_file_zip_blob_header_signatures() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let input_map: HashMap<String, Vec<u8>> = HashMap::from([
            (String::from("Hello.txt"), Vec::from(String::from("Capoo is Hungry.").as_bytes())),
            (String::from("Folder/Hi.csv"), Vec::from([1, 2, 3])),
        ]);

        let zip_blob = blob_factory_adapter.create_zip_binary(input_map);

        assert_eq!(237, zip_blob.len());

        //The consuming iterator used to create the file entry vector has arbitrary ordering
        if &zip_blob[39..55] == String::from("Capoo is Hungry.").as_bytes() {
            assert_eq!([0x50, 0x4B, 0x03, 0x04], &zip_blob[0..4]);
            assert_eq!([0x50, 0x4B, 0x01, 0x02], &zip_blob[101..105]);

            assert_eq!([0x50, 0x4B, 0x03, 0x04], &zip_blob[55..59]);
            assert_eq!([0x50, 0x4B, 0x01, 0x02], &zip_blob[156..160]);

            assert_eq!([0x50, 0x4B, 0x05, 0x06], &zip_blob[215..219]);
        } else {
            assert_eq!([0x50, 0x4B, 0x03, 0x04], &zip_blob[0..4]);
            assert_eq!([0x50, 0x4B, 0x01, 0x02], &zip_blob[101..105]);

            assert_eq!([0x50, 0x4B, 0x03, 0x04], &zip_blob[46..50]);
            assert_eq!([0x50, 0x4B, 0x01, 0x02], &zip_blob[160..164]);

            assert_eq!([0x50, 0x4B, 0x05, 0x06], &zip_blob[215..219]);
        }
    }
}