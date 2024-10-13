import streamlit as st
import requests
import regex as re

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

def main():
    if st.button('Submit'):
        if not re.match(evm_address_regex, address):
            st.error('Invalid EVM address. Please enter a valid address.')
            return

        # Set URL parameters
        st.query_params.chain = chain
        st.query_params.address = address

        demo()
        return

        url = config.backend_url + '/sample'
        response = requests.get(url)

        if response.status_code == 200:
            data = response.json()

            # 显示methods列表
            st.subheader('Methods')
            methods = data.get('methods', [])
            for method in methods:
                st.write(f"Name: {method['name']}")
                st.write(f"ID: {method['id']}")
                st.write(f"Signature: {method['signature']}")
                st.write(f"Sample TX: {method['sample_tx']}")
                st.write("---")

            # 显示events列表
            st.subheader('Events')
            events = data.get('events', [])
            for event in events:
                st.write(f"Name: {event['name']}")
                st.write(f"ID: {event['id']}")
                st.write(f"Signature: {event['signature']}")
                st.write(f"Sample TX: {event['sample_tx']}")
                st.write("---")
        else:
            st.error('Failed to fetch data. Please try again.')

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