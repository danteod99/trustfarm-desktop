from PIL import Image, ImageDraw, ImageFont
import os

def create_icon(size, output_path):
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Background: rounded rectangle effect with gradient-like color
    # Main color: teal/green (#00C9A7)
    bg_color = (0, 201, 167, 255)

    # Draw filled rounded rectangle
    margin = int(size * 0.08)
    radius = int(size * 0.18)
    draw.rounded_rectangle(
        [margin, margin, size - margin, size - margin],
        radius=radius,
        fill=bg_color
    )

    # Draw "TF" large text centered
    try:
        font_size_large = int(size * 0.45)
        font_large = ImageFont.truetype("C:/Windows/Fonts/arialbd.ttf", font_size_large)
    except:
        font_large = ImageFont.load_default()

    text_large = "TF"
    bbox = draw.textbbox((0, 0), text_large, font=font_large)
    tw, th = bbox[2] - bbox[0], bbox[3] - bbox[1]
    x = (size - tw) // 2
    y = int(size * 0.12)
    draw.text((x, y), text_large, fill=(255, 255, 255, 255), font=font_large)

    # Draw "trustfarm" smaller text below
    try:
        font_size_small = int(size * 0.13)
        font_small = ImageFont.truetype("C:/Windows/Fonts/arial.ttf", font_size_small)
    except:
        font_small = ImageFont.load_default()

    text_small = "trustfarm"
    bbox2 = draw.textbbox((0, 0), text_small, font=font_small)
    tw2 = bbox2[2] - bbox2[0]
    x2 = (size - tw2) // 2
    y2 = y + th + int(size * 0.05)
    draw.text((x2, y2), text_small, fill=(255, 255, 255, 230), font=font_small)

    img.save(output_path, 'PNG')
    print(f"Created {output_path} ({size}x{size})")

def create_ico(sizes, output_path):
    images = []
    for s in sizes:
        img = Image.new('RGBA', (s, s), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)
        margin = int(s * 0.08)
        radius = int(s * 0.18)
        bg_color = (0, 201, 167, 255)
        draw.rounded_rectangle([margin, margin, s - margin, s - margin], radius=radius, fill=bg_color)
        try:
            font_large = ImageFont.truetype("C:/Windows/Fonts/arialbd.ttf", int(s * 0.45))
        except:
            font_large = ImageFont.load_default()
        text = "TF"
        bbox = draw.textbbox((0, 0), text, font=font_large)
        tw, th = bbox[2] - bbox[0], bbox[3] - bbox[1]
        draw.text(((s - tw) // 2, int(s * 0.15)), text, fill=(255, 255, 255, 255), font=font_large)
        if s >= 64:
            try:
                font_small = ImageFont.truetype("C:/Windows/Fonts/arial.ttf", int(s * 0.13))
            except:
                font_small = ImageFont.load_default()
            bbox2 = draw.textbbox((0, 0), "trustfarm", font=font_small)
            tw2 = bbox2[2] - bbox2[0]
            draw.text(((s - tw2) // 2, int(s * 0.15) + th + int(s * 0.05)), "trustfarm", fill=(255, 255, 255, 230), font=font_small)
        images.append(img)
    images[0].save(output_path, format='ICO', sizes=[(s, s) for s in sizes], append_images=images[1:])
    print(f"Created {output_path}")

base = "C:/Users/Usuario/trustfarm-desktop"
icons_dir = f"{base}/src-tauri/icons"
assets_dir = f"{base}/src/assets"

# Tauri icons
icon_sizes = {
    "32x32.png": 32,
    "128x128.png": 128,
    "128x128@2x.png": 256,
    "icon.png": 512,
    "app-icon.png": 512,
    "Square30x30Logo.png": 30,
    "Square44x44Logo.png": 44,
    "Square71x71Logo.png": 71,
    "Square89x89Logo.png": 89,
    "Square107x107Logo.png": 107,
    "Square142x142Logo.png": 142,
    "Square150x150Logo.png": 150,
    "Square284x284Logo.png": 284,
    "Square310x310Logo.png": 310,
    "StoreLogo.png": 50,
}

for name, size in icon_sizes.items():
    create_icon(size, os.path.join(icons_dir, name))

# ICO file
create_ico([16, 32, 48, 64, 128, 256], os.path.join(icons_dir, "icon.ico"))

# Assets logos
create_icon(128, os.path.join(assets_dir, "app-icon.png"))
create_icon(128, os.path.join(assets_dir, "logo.png"))
create_icon(128, os.path.join(assets_dir, "logo_dark.png"))

# Root app-icon
create_icon(512, os.path.join(base, "app-icon.png"))

print("\nAll icons generated!")
