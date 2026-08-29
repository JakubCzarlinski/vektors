version="0.1.15"

mkdir -p registry
registry_version="v1.4.358"
generation_date="$(date +%Y-%m-%d)"
curl -o registry/video.xml "https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/${registry_version}/xml/video.xml"
curl -o registry/vk.xml "https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/${registry_version}/xml/vk.xml"

cargo run -p vk-codegen -- \
  --vk ./registry/vk.xml \
  --video ./registry/video.xml \
  --out ./vk \
  --crate-version "$version" \
  --readme ./README.md

readme_marker='<!-- generated registry metadata -->'
readme_metadata="As of ${generation_date}, bindings are generated from the Vulkan ${registry_version#v} registry."
if [ "$(grep -Fxc "$readme_marker" README.md)" -ne 1 ]; then
  echo "README must contain exactly one registry metadata marker" >&2
  exit 1
fi
sed -i "/^${readme_marker}\$/{n;s|^.*\$|${readme_metadata}|;}" README.md

cargo fmt && cargo clippy --fix --allow-dirty && cargo fmt
