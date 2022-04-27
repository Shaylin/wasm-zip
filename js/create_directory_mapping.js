 export function create_directory_mapping(directory_listing, folder_prefix) {
    const textEncoder = new TextEncoder();

    let directory_mapping = new Map();

    for (const file_name of Object.keys(directory_listing)) {
        const file_contents = directory_listing[file_name];
        const sub_file_name = get_sub_file_name(folder_prefix, file_name);

        if (is_nested_object(file_contents)) {
            const sub_directory_mapping = create_directory_mapping(file_contents, sub_file_name);
            directory_mapping = new Map([...directory_mapping, ...sub_directory_mapping])
        } else {
            let file_byte_array = get_file_contents_byte_array(textEncoder, file_contents);
            directory_mapping.set(sub_file_name, file_byte_array);
        }
    }

    return directory_mapping;
}

function is_nested_object(file_contents) {
    return typeof file_contents === "object" && !(file_contents instanceof Uint8Array);
}

function get_file_contents_byte_array(textEncoder, file_contents) {
    if (file_contents instanceof Uint8Array) {
        return file_contents;
    } else if (typeof file_contents == "string") {
        return textEncoder.encode(file_contents);
    } else {
        return textEncoder.encode("Unsupported file contents. Only string or Uint8Array is accepted.");
    }
}

function get_sub_file_name(folder_prefix, sub_directory) {
    if (folder_prefix) {
        return `${folder_prefix}/${sub_directory}`;
    }

    return sub_directory;
}