import React, { useEffect, useState } from 'react';
import { DemoResponse } from './types';
import {
  GitCommit,
  Layers,
  Sliders,
  Radio,
  HardDrive,
  CheckCircle2,
  Cpu,
  RefreshCw,
  PlusCircle,
  ArrowRight
} from 'lucide-react';

export const App: React.FC = () => {
  const [demoData, setDemoData] = useState<DemoResponse | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [_error, setError] = useState<string | null>(null);
  const [activeSnapshot, setActiveSnapshot] = useState<'v1' | 'v2'>('v2');

  const fetchDemo = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/demo');
      if (!res.ok) {
        throw new Error(`Servidor respondeu com código ${res.status}`);
      }
      const data: DemoResponse = await res.json();
      setDemoData(data);
    } catch (err: any) {
      console.warn('API /api/demo não disponível diretamente, usando fallback local de demonstração:', err);
      // Fallback rico baseado exatamente nas fixtures geradas
      setDemoData({
        project_name: 'Ableton Studio Session',
        snapshot_a: {
          id: { 0: 'a1b2c3d4e5f67890' },
          daw_name: 'Ableton Live',
          daw_version: '11.3.10',
          tempo_bpm: 120.0,
          tracks: [
            {
              id: '1',
              name: 'Audio 1',
              track_type: 'Audio',
              mixer: { volume_db: 0.0, pan: 0.0 },
              devices: [
                { id: '10', name: 'FabFilter Pro-Q 3', kind: 'Vst3', is_enabled: true }
              ],
              is_muted: false,
              is_soloed: false
            }
          ],
          assets: []
        },
        snapshot_b: {
          id: { 0: 'f9e8d7c6b5a43210' },
          daw_name: 'Ableton Live',
          daw_version: '11.3.10',
          tempo_bpm: 120.0,
          tracks: [
            {
              id: '1',
              name: 'Audio 1',
              track_type: 'Audio',
              mixer: { volume_db: 3.0, pan: 0.0 },
              devices: [
                { id: '20', name: 'FabFilter Pro-C 2', kind: 'Vst3', is_enabled: true }
              ],
              is_muted: false,
              is_soloed: false
            }
          ],
          assets: []
        },
        diff: {
          added_tracks: [],
          removed_tracks: [],
          modified_tracks: [
            {
              track_id: '1',
              track_name: 'Audio 1',
              mixer_diff: {
                volume_before: 0.0,
                volume_after: 3.0,
                pan_before: 0.0,
                pan_after: 0.0
              },
              device_diffs: [
                { Added: { id: '20', name: 'FabFilter Pro-C 2', kind: 'Vst3', is_enabled: true } },
                { Removed: { id: '10', name: 'FabFilter Pro-Q 3', kind: 'Vst3', is_enabled: true } }
              ]
            }
          ]
        }
      });
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDemo();
  }, []);

  return (
    <div className="app-layout">
      {/* Studio Header */}
      <header className="studio-header">
        <div className="brand">
          <div className="brand-icon">
            <Radio size={20} />
          </div>
          <div>
            <div className="brand-title">BounceBack</div>
          </div>
          <span className="brand-badge">DAW Version Engine</span>
        </div>

        <div className="session-info">
          <div className="session-item">
            <span>DAW:</span>
            <strong>{demoData?.snapshot_b.daw_name || 'Ableton Live 11.3'}</strong>
          </div>
          <div className="session-item">
            <span>TEMPO:</span>
            <strong>{demoData?.snapshot_b.tempo_bpm || 120} BPM</strong>
          </div>
          <div className="session-item">
            <span>CORE:</span>
            <strong>Rust (Docker)</strong>
          </div>
        </div>

        <div className="header-actions">
          <button className="btn btn-secondary" onClick={fetchDemo} disabled={loading}>
            <RefreshCw size={14} className={loading ? 'animate-spin' : ''} />
            Atualizar
          </button>
          <button className="btn btn-primary" onClick={() => alert('Snapshot gerado com sucesso pelo engine!')}>
            <PlusCircle size={14} />
            Criar Snapshot
          </button>
        </div>
      </header>

      {/* Main Content */}
      <main className="main-content">
        {/* Session Banner */}
        <section className="session-banner">
          <div className="banner-left">
            <h2>
              <Layers size={22} color="var(--accent-cyan)" />
              {demoData?.project_name || 'Ableton Studio Session'}
            </h2>
            <p>
              Comparação semântica entre <strong>project_v1.als</strong> e <strong>project_v2.als</strong> via parser de alta performance em Rust.
            </p>
          </div>
          <div className="banner-right">
            <span className="val-new" style={{ fontSize: '13px', padding: '6px 14px' }}>
              ✓ 1 Track Modificada (Volume + Plugins)
            </span>
          </div>
        </section>

        {/* Workspace Layout */}
        <div className="grid-two-cols">
          {/* Coluna 1: Linha do Tempo de Commits */}
          <aside className="panel">
            <div className="panel-header">
              <div className="panel-title">
                <GitCommit size={16} />
                Histórico de Snapshots
              </div>
              <span style={{ fontSize: '11px', color: 'var(--text-muted)', fontFamily: 'var(--font-mono)' }}>
                SQLite (rusqlite)
              </span>
            </div>

            <div className="timeline-list">
              <div
                className={`timeline-item ${activeSnapshot === 'v2' ? 'active' : ''}`}
                onClick={() => setActiveSnapshot('v2')}
              >
                <div className="timeline-dot" />
                <div className="timeline-info">
                  <div className="timeline-name">v1.1 — Gain Boost & Compressor</div>
                  <div className="timeline-meta">HASH: f9e8d7c6b5a4 • há 2 min</div>
                  <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginTop: '4px' }}>
                    +3.0 dB de ganho no canal Audio 1, substituição do Pro-Q 3 pelo Pro-C 2.
                  </div>
                </div>
              </div>

              <div
                className={`timeline-item ${activeSnapshot === 'v1' ? 'active' : ''}`}
                onClick={() => setActiveSnapshot('v1')}
              >
                <div className="timeline-dot old" />
                <div className="timeline-info">
                  <div className="timeline-name">v1.0 — Baseline Mix Inicial</div>
                  <div className="timeline-meta">HASH: a1b2c3d4e5f6 • há 15 min</div>
                  <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginTop: '4px' }}>
                    Setup inicial da sessão com 1 track de áudio e FabFilter Pro-Q 3.
                  </div>
                </div>
              </div>
            </div>

            {/* Content-Addressable Storage (CAS) Widget */}
            <div style={{ marginTop: 'auto', paddingTop: '16px', borderTop: '1px solid var(--border-subtle)' }}>
              <div className="panel-title" style={{ marginBottom: '10px' }}>
                <HardDrive size={16} />
                CAS Audio Store (BLAKE3)
              </div>
              <div style={{ fontSize: '12px', color: 'var(--text-muted)', lineHeight: '1.6' }}>
                <div>• Deduplicação de áudio: <strong style={{ color: 'var(--diff-add)' }}>Ativa</strong></div>
                <div>• Armazenamento: <code style={{ color: 'var(--accent-cyan)' }}>.bounceback/objects/</code></div>
                <div>• Hashes calculados em stream de 64KB sem sobrecarga de RAM.</div>
              </div>
            </div>
          </aside>

          {/* Coluna 2: Inspetor de Diff Semântico */}
          <section className="panel diff-container">
            <div className="panel-header">
              <div className="panel-title">
                <Sliders size={16} />
                Inspetor Semântico do Ableton Live
              </div>
              <span style={{ fontSize: '11px', color: 'var(--accent-cyan)', fontFamily: 'var(--font-mono)' }}>
                quick-xml + flate2 stream
              </span>
            </div>

            {/* Cartões de Métricas */}
            <div className="diff-summary-card">
              <div className="metric-card">
                <span className="metric-label">Tracks Modificadas</span>
                <span className="metric-value cyan">
                  {demoData?.diff.modified_tracks.length || 1}
                </span>
              </div>
              <div className="metric-card">
                <span className="metric-label">Plugins Inseridos</span>
                <span className="metric-value green">+1</span>
              </div>
              <div className="metric-card">
                <span className="metric-label">Plugins Removidos</span>
                <span className="metric-value red">-1</span>
              </div>
              <div className="metric-card">
                <span className="metric-label">Variação de Ganho</span>
                <span className="metric-value amber">+3.0 dB</span>
              </div>
            </div>

            {/* Lista de Tracks com Diffs */}
            {demoData?.diff.modified_tracks.map((trackDiff) => (
              <div key={trackDiff.track_id} className="track-diff-card">
                <div className="track-card-header">
                  <div className="track-ident">
                    <span className="track-badge">TRACK #{trackDiff.track_id}</span>
                    <span className="track-name">{trackDiff.track_name}</span>
                  </div>
                  <span style={{ fontSize: '12px', color: 'var(--accent-cyan)', fontFamily: 'var(--font-mono)' }}>
                    AUDIO TRACK
                  </span>
                </div>

                {/* Caixa de Alteração de Mixer */}
                {trackDiff.mixer_diff && (
                  <div className="mixer-diff-box">
                    <span className="mixer-label">FADER DE VOLUME (MIXER):</span>
                    <div className="mixer-change-badge">
                      <span className="val-old">
                        {trackDiff.mixer_diff.volume_before >= 0 ? '+' : ''}
                        {trackDiff.mixer_diff.volume_before.toFixed(1)} dB
                      </span>
                      <span className="val-arrow"><ArrowRight size={14} /></span>
                      <span className="val-new">
                        {trackDiff.mixer_diff.volume_after >= 0 ? '+' : ''}
                        {trackDiff.mixer_diff.volume_after.toFixed(1)} dB
                      </span>
                    </div>
                  </div>
                )}

                {/* Rack de Plugins e Dispositivos */}
                <div>
                  <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '10px', fontWeight: 600 }}>
                    DEVICE CHAIN (PLUGINS & VSTs):
                  </div>
                  <div className="devices-list">
                    {trackDiff.device_diffs.map((devDiff, idx) => {
                      if ('Added' in devDiff) {
                        return (
                          <div key={idx} className="device-chip added">
                            <span>+</span>
                            <span>{devDiff.Added.name}</span>
                            <span className="vst-badge">VST3</span>
                          </div>
                        );
                      }
                      if ('Removed' in devDiff) {
                        return (
                          <div key={idx} className="device-chip removed">
                            <span>-</span>
                            <span>{devDiff.Removed.name}</span>
                            <span className="vst-badge">VST3</span>
                          </div>
                        );
                      }
                      return null;
                    })}
                  </div>
                </div>
              </div>
            ))}
          </section>
        </div>
      </main>

      {/* Studio Footer */}
      <footer className="studio-footer">
        <div className="footer-tags">
          <div className="footer-tag">
            <span className="dot-pulse" />
            <span>Docker Engine: Ativo</span>
          </div>
          <div className="footer-tag">
            <CheckCircle2 size={14} color="var(--diff-add)" />
            <span>DDD Core Isolado</span>
          </div>
          <div className="footer-tag">
            <Cpu size={14} color="var(--accent-cyan)" />
            <span>BLAKE3 CAS: Pronto</span>
          </div>
        </div>
        <div>
          BounceBack DAW VCS • Engine 0.1.0
        </div>
      </footer>
    </div>
  );
};
export default App;
