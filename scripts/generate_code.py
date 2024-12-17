import requests

# GitHub repository details
owner = "ossf"
repo = "scorecard"
path = "probes"
url = f"https://api.github.com/repos/{owner}/{repo}/contents/{path}"

# GitHub API requires a User-Agent header
headers = {
    "User-Agent": "Python-Directory-Fetcher"
}

def get_directories(url):
    try:
        # Send a GET request to GitHub API
        response = requests.get(url, headers=headers)
        response.raise_for_status()  # Raise error for HTTP issues
        
        # Parse the JSON response
        contents = response.json()
        
        # Filter directories (type == "dir")
        directories = [item['name'] for item in contents if item['type'] == 'dir']
        
        # Display directories
        print("Directories in 'probes':")
        for directory in directories:
            print(f"- {directory}")
    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")

# Fetch and display directories
get_directories(url)
