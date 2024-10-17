import streamlit as st
import requests
import regex as re

from st_social_media_links import SocialMediaIcons
from config import config

def main():
    # Set page configuration
    st.set_page_config(page_title='EVM Transaction Sampler', layout='wide')
    
    # Title
    st.title('EVM Transaction Sampler')

    # Initialize session state
    init_session_state()

    # Render input section
    render_input_section()

    # Render social media links
    render_social_media_links()

def init_session_state():
    if 'button_disabled' not in st.session_state:
        st.session_state.button_disabled = False

def render_input_section():
    # Define available chain options
    chain_options = {"Ethereum": "eth", "Arbitrum": "arbitrum"}
    
    # Get default values from query parameters
    query_params = st.query_params
    default_chain_value = query_params.get('chain', ['eth'])[0]
    default_chain = next((name for name, value in chain_options.items() if value == default_chain_value), "Ethereum")
    default_address = query_params.get('address', [''])[0]

    # Display chain selection dropdown
    chain = st.selectbox(
        'Select Chain',
        options=list(chain_options.keys()),
        index=list(chain_options.keys()).index(default_chain),
        help="Currently only Ethereum is supported.",
        disabled=True
    )

    # Input address
    address = st.text_input('Enter Address', value=default_address)

    # Submit button
    st.button('Submit', on_click=submit, args=(chain_options[chain], address), disabled=st.session_state.button_disabled)

def submit(chain, address):
    st.session_state.button_disabled = True
    evm_address_regex = r'^0x[a-fA-F0-9]{40}$'
    
    with st.spinner('Loading...'):
        if not re.match(evm_address_regex, address):
            st.error('Please enter a valid EVM address.')
            st.session_state.button_disabled = False
            return

        # Update query parameters
        st.query_params.chain = chain
        st.query_params.address =address

        # Request data
        url = config.backend_url + '/sample'
        try:
            response = requests.get(url, params={"chain": chain, "address": address}, timeout=60)
            response.raise_for_status()
            data = response.json()
        except requests.exceptions.RequestException as e:
            st.error(f"Error occurred while requesting data: {e}")
            st.session_state.button_disabled = False
            return

        error_message = data.get("error_message")
        if error_message:
            st.warning(f"**Error Message:** {error_message}")
        else:
            display_data(data.get("data", []))
            st.balloons()

    st.session_state.button_disabled = False

def display_data(data):
    st.markdown("---")  # Divider

    if not data:
        st.info("No transaction data available to display.")
        return

    for idx, item in enumerate(data, start=1):
        tx_hash = item.get('tx_hash', 'Unknown Hash')
        with st.expander(f"Transaction {idx}: {tx_hash}"):
            display_json(item)

def display_json(data, indent=0):
    if isinstance(data, dict):
        for key, value in data.items():
            title = snake_to_title(key)
            if key == 'logs':
                st.markdown(f"**{title}:**")
                display_logs(value, indent + 1)
            elif isinstance(value, (dict, list)):
                st.markdown(f"**{title}:**")
                display_json(value, indent + 1)
            else:
                st.write(f"**{title}:** {value}")
    elif isinstance(data, list):
        for idx, item in enumerate(data, start=1):
            st.markdown(f"**Item {idx}:**")
            display_json(item, indent + 1)
    else:
        st.write(data)

def display_logs(logs, indent=0):
    for idx, log in enumerate(logs, start=1):
        st.markdown(f"**Log {idx}:**")
        st.write(f"**Event ID:** {log[0]}")
        st.write(f"**Event Signature:** {log[1]}")

def snake_to_title(snake_str):
    components = snake_str.split('_')
    titled = ' '.join(x.capitalize() for x in components)
    return titled

def render_social_media_links():
    social_media_links = [
        "https://x.com/ioogleio",
        "https://www.youtube.com/@ioogleio",
        "https://github.com/ioogle",
        "https://medium.com/@ioogle",
        "https://www.linkedin.com/in/hui-zeng-6a18381b6/",
    ]
    social_media_icons = SocialMediaIcons(social_media_links)
    social_media_icons.render()

if __name__ == "__main__":
    main()