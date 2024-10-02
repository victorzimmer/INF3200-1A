import json
import sys

def main():
    if len(sys.argv) != 2:
        print("Usage: python json_to_spaces.py <json_string>")
        return

    try:
        input_json = sys.argv[1]
        data = json.loads(input_json)

        # Check if the input is a list of strings
        if not all(isinstance(item, str) for item in data):
            print("Error: Input must be a list of strings")
            return

        output_list = [' '.join(data)]
        print(*output_list, sep='\n')

    except json.JSONDecodeError:
        print("Error: Invalid JSON input")

if __name__ == "__main__":
    main()
