import streamlit as st
import requests
import regex as re

from st_social_media_links import SocialMediaIcons

from config import config

# Title
st.title('EVM Transaction Sampler')

# Retrieve query parameters
try:
    default_chain = st.query_params.chain
    default_address = st.query_params.address
except Exception as e:
    default_chain = "eth"
    default_address = ""

# Chain selection dropdown
chain_options = {"Ethereum": "eth", "Arbitrum": "arbitrum"}
default_chain = next((name for name, value in chain_options.items() if value == default_chain), "Ethereum")
chain = st.selectbox(
    'Chain', 
    options=list(chain_options.keys()), 
    index=list(chain_options.keys()).index(default_chain), 
    help="Currently, only Ethereum is supported.",
    disabled=True,
    )

# Text inputs
address = st.text_input('Address', value=default_address)

evm_address_regex = r'^0x[a-fA-F0-9]{40}$'

# 定义转换函数
def snake_to_title(snake_str):
    components = snake_str.split('_')
    titled = ' '.join(x.capitalize() for x in components)
    return titled

def display_json(data, indent=0):
    spacing = " " * (indent * 4)
    if isinstance(data, dict):
        for key, value in data.items():
            title = snake_to_title(key)
            if isinstance(value, (dict, list)):
                st.markdown(f"**{title}:**")
                display_json(value, indent + 1)
            else:
                st.write(f"**{title}:** {value}")
    elif isinstance(data, list):
        st.markdown(f"**Event ID:**")
        display_json(data[0], indent + 1)
        st.markdown(f"**Event Signature:**")
        display_json(data[1], indent + 1)
    else:
        st.write(data)

if 'button_disabled' not in st.session_state:
    st.session_state.button_disabled = False

def submit():
    st.session_state.button_disabled = True
    with st.spinner('Loading...'):
        if not re.match(evm_address_regex, address):
            st.error('Invalid EVM address. Please enter a valid address.')
            return

        # Set URL parameters
        st.query_params.chain = chain
        st.query_params.address = address

        url = config.backend_url + '/sample'
        response = requests.get(url, params={"chain": chain, "address": address})

        data = response.json()
        error_message = data.get("error_message")

        if error_message:
            st.warning(f"**Error Message:** {error_message}")
        else:
            st.markdown("---")  # 分割线

            data = data.get("data", [])

            if not data:
                st.write("No transaction data to display.")
            else:
                for idx, item in enumerate(data, start=1):
                    with st.expander(f"Transaction {idx}: {item.get('tx_hash')}"):
                        display_json(item)
            st.balloons()

    st.session_state.button_disabled = False

st.button('Submit', on_click=submit, disabled=st.session_state.button_disabled)

social_media_links = [
    "https://x.com/ioogleio",
    "https://www.youtube.com/@ioogleio",
    "https://github.com/ioogle",
    "https://medium.com/@ioogle",
    "https://www.linkedin.com/in/hui-zeng-6a18381b6/",
]

social_media_icons = SocialMediaIcons(social_media_links)

social_media_icons.render()