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
    help="Currently, only Ethreum is supported.",
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
        for idx, item in enumerate(data, start=1):
            st.markdown(f"**项 {idx}:**")
            display_json(item, indent + 1)
    else:
        st.write(data)

def main():
    if st.button('Submit'):
        if not re.match(evm_address_regex, address):
            st.error('Invalid EVM address. Please enter a valid address.')
            return

        # Set URL parameters
        st.query_params.chain = chain
        st.query_params.address = address

        url = config.backend_url + '/sample'
        response = requests.get(url, params={"chain": chain, "address": address})

        st.header("Response status")
        status = response.get("status")
        error_message = response.get("error_message")

        st.write(f"**Status:** {status}")
        if error_message:
            st.write(f"**Error Message:** {error_message}")
        else:
            st.write("**Error Message:** None")

        st.markdown("---")  # 分割线

        data = response.get("data", [])

        if not data:
            st.write("No transaction data to display.")
        else:
            for idx, item in enumerate(data, start=1):
                with st.expander(f"Transaction {idx}: {item.get('tx_hash')}"):
                    display_json(item)

def demo():
    # Demo data
    data = {
        'methods': [
            {
                'name': 'transfer',
                'id': '0x12345678',
                'signature': 'transfer(address,uint256)',
                'sample_tx': '0xabcdef'
            }
        ],
        'events': [
            {
                'name': 'Transfer',
                'id': '0x87654321',
                'signature': 'Transfer(address,address,uint256)',
                'sample_tx': '0x123abc'
            }
        ]
    }

    # Display methods list
    st.subheader('Methods')
    methods = data.get('methods', [])
    for method in methods:
        st.write(f"Name: {method['name']}")
        st.write(f"ID: {method['id']}")
        st.write(f"Signature: {method['signature']}")
        st.write(f"Sample TX: {method['sample_tx']}")
        st.write("---")

    # Display events list
    st.subheader('Events')
    events = data.get('events', [])
    for event in events:
        st.write(f"Name: {event['name']}")
        st.write(f"ID: {event['id']}")
        st.write(f"Signature: {event['signature']}")
        st.write(f"Sample TX: {event['sample_tx']}")
        st.write("---")

main()

social_media_links = [
    "https://x.com/ioogleio",
    "https://www.youtube.com/@ioogleio",
    "https://github.com/ioogle",
    "https://medium.com/@ioogle",
    "https://www.linkedin.com/in/hui-zeng-6a18381b6/",
]

social_media_icons = SocialMediaIcons(social_media_links)

social_media_icons.render()