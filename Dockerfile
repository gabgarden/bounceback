FROM rust:1.85-slim

WORKDIR /workspace

# Instala dependências mínimas do sistema
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    sqlite3 \
    libsqlite3-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copia arquivos do projeto
COPY . .

# Comando padrão: executa os testes do core
CMD ["cargo", "test", "--workspace", "--verbose"]
