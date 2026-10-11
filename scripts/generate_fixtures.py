import gzip
from pathlib import Path

PROJECT_A_XML = """<?xml version="1.0" encoding="UTF-8"?>
<Ableton MajorVersion="5" MinorVersion="11.0_11300" SchemaChangeCount="3" Creator="Ableton Live 11.3.10">
  <LiveSet>
    <Tracks>
      <AudioTrack Id="1">
        <Name>
          <EffectiveName Value="Audio 1" />
        </Name>
        <DeviceChain>
          <Mixer>
            <Volume>
              <Manual Value="0.0" />
            </Volume>
            <Pan>
              <Manual Value="0.0" />
            </Pan>
          </Mixer>
          <DeviceChain>
            <Devices>
              <PluginDevice Id="10">
                <PluginDesc>
                  <Vst3PluginInfo>
                    <Name Value="FabFilter Pro-Q 3" />
                  </Vst3PluginInfo>
                </PluginDesc>
              </PluginDevice>
            </Devices>
          </DeviceChain>
        </DeviceChain>
      </AudioTrack>
    </Tracks>
  </LiveSet>
</Ableton>
"""

PROJECT_B_XML = """<?xml version="1.0" encoding="UTF-8"?>
<Ableton MajorVersion="5" MinorVersion="11.0_11300" SchemaChangeCount="3" Creator="Ableton Live 11.3.10">
  <LiveSet>
    <Tracks>
      <AudioTrack Id="1">
        <Name>
          <EffectiveName Value="Audio 1" />
        </Name>
        <DeviceChain>
          <Mixer>
            <Volume>
              <Manual Value="3.0" />
            </Volume>
            <Pan>
              <Manual Value="0.0" />
            </Pan>
          </Mixer>
          <DeviceChain>
            <Devices>
              <PluginDevice Id="20">
                <PluginDesc>
                  <Vst3PluginInfo>
                    <Name Value="FabFilter Pro-C 2" />
                  </Vst3PluginInfo>
                </PluginDesc>
              </PluginDevice>
            </Devices>
          </DeviceChain>
        </DeviceChain>
      </AudioTrack>
    </Tracks>
  </LiveSet>
</Ableton>
"""

def create_als_fixture(filepath: Path, xml_content: str):
    filepath.parent.mkdir(parents=True, exist_ok=True)
    with gzip.open(filepath, "wb") as f:
        f.write(xml_content.strip().encode("utf-8"))
    print(f"Fixture gerada: {filepath} ({filepath.stat().st_size} bytes)")

def main():
    target_dirs = [
        Path("tests/fixtures/ableton"),
        Path("crates/bounceback-core/tests/fixtures/ableton"),
    ]
    
    for fixtures_dir in target_dirs:
        create_als_fixture(fixtures_dir / "project_v1.als", PROJECT_A_XML)
        create_als_fixture(fixtures_dir / "project_v2.als", PROJECT_B_XML)
        create_als_fixture(fixtures_dir / "minimal_v11.als", PROJECT_A_XML)
        create_als_fixture(fixtures_dir / "minimal_v11_modified.als", PROJECT_B_XML)
    
    print("Todas as fixtures do Ableton geradas em todas as localizações!")

if __name__ == "__main__":
    main()
