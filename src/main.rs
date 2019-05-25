use std::fs::File;

fn main() {
    let client = reqwest::Client::new();

    let application_endpoint = env!("APPLICATION_ENDPOINT");
    let application_id = env!("APPLICATION_ID");
    let file_path = String::from(env!("FILE_PATH")).split(",").collect();

    for path in file_path.iter() {
        let file = File::open(path).unwrap();
        let mut rdr = csv::Reader::from_reader(file);
        let headers = rdr.headers().unwrap().clone();

        for result in rdr.records() {
            let record = result.unwrap();

            let mut push_data = String::from("{");

            for i in 0..headers.len() {
                push_data.push_str(format!("\"{}\": \"{}\"", headers.get(i).unwrap(), record.get(i).unwrap()).as_str());
            }

            push_data.push_str("}");

            client.post(format!("{}/classes/{}", application_endpoint, path.replace(".csv", "")).as_str())
                .header("X-Parse-Application-Id", application_id)
                .body(push_data)
                .send();
        }
    }
}
