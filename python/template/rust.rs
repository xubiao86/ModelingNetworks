{%- if header or query or body -%}
use std::collections::HashMap;

{% endif -%}

use super::RequestTrait;

pub struct {{ struct_name }};

impl RequestTrait for {{ struct_name }} {
    fn http_url(&self) -> String {
        "{{ url }}".to_string()
    }

    {%- if path %}

    fn http_path(&self) -> String {
        "{{ path }}".to_string()
    }
    {%- endif %}

    fn http_method(&self) -> String {
        "{{ method }}".to_string()
    }

    {%- if header %}

    fn http_header(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        {%- for key, value in header.items() %}
        map.insert("{{ key }}".to_string(), "{{ value }}".to_string());
        {%- endfor %}
        map
    }
    {%- endif %}

    {%- if query %}

    fn http_query(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        {%- for key, value in query.items() %}
        map.insert("{{ key }}".to_string(), "{{ value }}".to_string());
        {%- endfor %}
        map
    }
    {%- endif %}

    {%- if body %}

    fn http_body(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        {%- for key, value in body.items() %}
        map.insert("{{ key }}".to_string(), "{{ value }}".to_string());
        {%- endfor %}
        map
    }
    {%- endif %}
}
