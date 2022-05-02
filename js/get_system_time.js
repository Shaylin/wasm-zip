export function get_system_time() {
    const date = new Date();

    let system_time_array = new Uint16Array(6);
    system_time_array[0] = date.getHours();
    system_time_array[1] = date.getMinutes();
    system_time_array[2] = date.getSeconds();
    system_time_array[3] = date.getDate();
    system_time_array[4] = date.getMonth();
    system_time_array[5] = date.getFullYear();

    return system_time_array;
}