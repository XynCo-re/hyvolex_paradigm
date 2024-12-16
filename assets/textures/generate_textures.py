from PIL import Image
import numpy as np
import os

def create_cel_ramp(width=256, height=1):
    # Create a new image with RGB mode
    img = Image.new('RGB', (width, height))
    pixels = img.load()
    
    # Define our 3 cel-shading levels
    levels = [
        (102, 102, 102),  # Dark tone (0.4, 0.4, 0.4)
        (178, 178, 178),  # Mid tone (0.7, 0.7, 0.7)
        (255, 255, 255)   # Light tone (1.0, 1.0, 1.0)
    ]
    
    # Fill the image with our cel-shading levels
    section_width = width // 3
    for x in range(width):
        level_idx = min(x // section_width, 2)
        pixels[x, 0] = levels[level_idx]
    
    return img

def generate_perlin_noise(width=512, height=512):
    def generate_octave(width, height, scale):
        noise = np.random.rand(height//scale, width//scale) * 255
        return Image.fromarray(noise.repeat(scale, axis=0).repeat(scale, axis=1).astype(np.uint8))
    
    # Generate multiple octaves of noise
    scales = [4, 8, 16, 32]
    weights = [1.0, 0.5, 0.25, 0.125]
    
    base = np.zeros((height, width), dtype=np.float32)
    
    for scale, weight in zip(scales, weights):
        octave = generate_octave(width, height, scale)
        base += np.array(octave, dtype=np.float32) * weight
    
    # Normalize to 0-255 range
    base = ((base - base.min()) * (255.0 / (base.max() - base.min()))).astype(np.uint8)
    return Image.fromarray(base)

def main():
    # Create textures directory if it doesn't exist
    os.makedirs('assets/textures', exist_ok=True)
    
    # Generate and save cel ramp texture
    ramp = create_cel_ramp()
    ramp.save('assets/textures/ramp.png')
    
    # Generate and save noise texture
    noise = generate_perlin_noise()
    noise.save('assets/textures/noise.png')
    
    print("Textures generated successfully!")

if __name__ == "__main__":
    main() 