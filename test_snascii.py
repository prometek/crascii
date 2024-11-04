import crascii

def main():
    # Define options
    options = crascii.PyOptions(
        columns=80,
        lines=None,
        color=True,
        charsets="default",
        output_path="output.png"
    )

    # Create an instance of PyASCIIImage
    ascii_image = crascii.PyASCIIImage("musk.jpg", options)

    # Perform the conversion
    ascii_image.convert()

    print("Conversion completed. Output saved to 'output.png'.")

if __name__ == "__main__":
    main()

