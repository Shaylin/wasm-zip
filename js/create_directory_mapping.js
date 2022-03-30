// TODO: Remember to add export back
function create_directory_mapping(directory_listing, folder_prefix) {
    let directory_mapping = new Map();

    for (const [file_name, file_contents] of Object.entries(directory_listing)) {
        const sub_file_name = get_sub_file_name(folder_prefix, file_name);

        if (typeof file_contents === "object") {
            const sub_directory_mapping = create_directory_mapping(file_contents, sub_file_name);
            directory_mapping = new Map([...directory_mapping, ...sub_directory_mapping])
        } else {
            directory_mapping.set(sub_file_name, file_contents);
        }
    }

    return directory_mapping;
}

function get_sub_file_name(folder_prefix, sub_directory) {
    if (folder_prefix) {
        return `${folder_prefix}/${sub_directory}`;
    }

    return sub_directory;
}

const test_object = {
    "myFolder": {
        "tutu.json": JSON.stringify({rabbit: true, description: "likes capoo"}),
        "subFolder": {
            "yow.jpg": "pretend this is some binary"
        }
    },
    "bugcat.txt": "Capoo The BugCat"
}

console.log(create_directory_mapping(test_object));