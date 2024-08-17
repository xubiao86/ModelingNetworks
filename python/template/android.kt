package {{ package_name }}

import com.succulents.network.RequestModel

class {{ class_name }} : RequestModel() {
    override val url: String
        get() = "{{ url }}"

    {%- if path %}

    override val path: String
        get() = "{{ path }}"
    {%- endif %}

    override val method: String
        get() = "{{ method }}"

    {%- if header %}

    override val header: Map<String, String>
        get() = mapOf(
            {%- for key, value in header.items() %}
            "{{ key }}" to "{{ value }}"{% if not loop.last %},{% endif %}
            {%- endfor %}
        )
    {%- endif %}
    
    {%- if query %}

    override val query: Map<String, String>
        get() = mapOf(
            {%- for key, value in query.items() %}
            "{{ key }}" to "{{ value }}"{% if not loop.last %},{% endif %}
            {%- endfor %}
        )
    {%- endif %}
    
    {%- if body %}

    override val body: Map<String, String>
        get() = mapOf(
            {%- for key, value in body.items() %}
            "{{ key }}" to "{{ value }}"{% if not loop.last %},{% endif %}
            {%- endfor %}
        )
    {%- endif %}
}
