// TODO: Remember to add export back
function create_directory_mapping(directory_listing, folder_prefix) {
    let directory_mapping = new Map();

    for (const [key, value] of Object.entries(directory_listing)) {
        const sub_value_name = get_sub_value_name(folder_prefix, key);

        if (typeof value === "object") {
            const sub_directory_mapping = create_directory_mapping(value, sub_value_name);
            directory_mapping = new Map([...directory_mapping, ...sub_directory_mapping])
        } else {
            directory_mapping.set(sub_value_name, value);
        }
    }

    return directory_mapping;
}

function get_sub_value_name(folder_prefix, sub_directory) {
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