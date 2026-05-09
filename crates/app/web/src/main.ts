import { invoke } from "@tauri-apps/api/core";
import "./style.css" with {
	type: "css",
};

type BondListItem = {
	id: string;
	name: string | null;
	source: string;
	target: string;
	createdAt: string;
	status: string;
	metadataCount: number;
};

const app = document.querySelector<HTMLDivElement>("#app");
if (!app) {
	throw new Error("Missing app root");
}

app.innerHTML = `
  <div class="shell">
    <section class="panel home-panel">
      <button class="home-tile" type="button" data-view="bonds">
        <span class="home-title">HOME</span>
        <span class="home-subtitle">Toggle current bonds view</span>
      </button>
    </section>

    <header class="panel hero-panel">
      <div class="eyebrow">Bonds Desktop</div>
      <h1>Bond Viewer</h1>
      <p>Browse tracked bonds from the same SQLite store used by the CLI.</p>
    </header>

    <aside class="sidebar">
      <button class="panel menu-item is-active" type="button" data-view="bonds">
        Current Bonds
      </button>
      <button class="panel menu-item" type="button" disabled>
        Search
      </button>
      <button class="panel menu-item" type="button" disabled>
        Bookmarks
      </button>
      <button class="panel menu-item" type="button" disabled>
        Settings
      </button>
    </aside>

    <main class="panel main-view">
      <div class="panel-heading">
        <div>
          <div class="eyebrow">Main view</div>
          <h2>Current bonds</h2>
        </div>
        <div class="panel-stats">
          <span data-count>0 bonds</span>
          <span data-last-sync>not loaded</span>
        </div>
      </div>

      <section class="bond-list" data-bond-list></section>
    </main>

    <footer class="panel toolbar">
      <div class="toolbar-left">
        <button class="toolbar-button" type="button" data-action="refresh">
          Refresh
        </button>
      </div>
      <div class="toolbar-right" data-status>
        Ready
      </div>
    </footer>
  </div>
`;

const listEl = document.querySelector<HTMLElement>("[data-bond-list]")!;
const countEl = document.querySelector<HTMLElement>("[data-count]")!;
const lastSyncEl = document.querySelector<HTMLElement>("[data-last-sync]")!;
const statusEl = document.querySelector<HTMLElement>("[data-status]")!;

function escapeHtml(value: string): string {
	return value
		.replaceAll("&", "&amp;")
		.replaceAll("<", "&lt;")
		.replaceAll(">", "&gt;")
		.replaceAll('"', "&quot;")
		.replaceAll("'", "&#39;");
}

function formatDate(value: string): string {
	const date = new Date(value);
	return Number.isNaN(date.valueOf()) ? value : date.toLocaleString();
}

function shortId(id: string): string {
	return id.slice(0, 8);
}

function renderBonds(bonds: BondListItem[]): void {
	countEl.textContent = `${bonds.length} bond${bonds.length === 1 ? "" : "s"}`;
	lastSyncEl.textContent = `Updated ${new Date().toLocaleTimeString()}`;

	if (bonds.length === 0) {
		listEl.innerHTML = `
      <article class="empty-state">
        <h3>No bonds yet</h3>
        <p>Create your first bond from the terminal with <code>bond add &lt;source&gt;</code>.</p>
      </article>
    `;
		return;
	}

	listEl.innerHTML = bonds
		.map((bond) => {
			const label = escapeHtml(bond.name ?? shortId(bond.id));
			const source = escapeHtml(bond.source);
			const target = escapeHtml(bond.target);
			const status = escapeHtml(bond.status);
			const id = escapeHtml(shortId(bond.id));

			return `
        <article class="bond-card">
          <div class="bond-card-top">
            <div>
              <h3>${label}</h3>
              <div class="bond-id">${id}</div>
            </div>
            <span class="status-pill status-${status}">
              ${status}
            </span>
          </div>

          <dl class="bond-meta">
            <div>
              <dt>Source</dt>
              <dd title="${source}">${source}</dd>
            </div>
            <div>
              <dt>Target</dt>
              <dd title="${target}">${target}</dd>
            </div>
            <div>
              <dt>Created</dt>
              <dd>${escapeHtml(formatDate(bond.createdAt))}</dd>
            </div>
            <div>
              <dt>Metadata</dt>
              <dd>${bond.metadataCount}</dd>
            </div>
          </dl>
        </article>
      `;
		})
		.join("");
}

async function loadBonds(): Promise<void> {
	statusEl.textContent = "Loading bonds…";

	try {
		const bonds = await invoke<BondListItem[]>("list_bonds");
		renderBonds(bonds);
		statusEl.textContent = "Bonds loaded";
	} catch (error) {
		const message = escapeHtml(String(error));
		listEl.innerHTML = `
      <article class="empty-state error-state">
        <h3>Unable to load bonds</h3>
        <p>${message}</p>
      </article>
    `;
		countEl.textContent = "0 bonds";
		lastSyncEl.textContent = "load failed";
		statusEl.textContent = "Load failed";
	}
}

document
	.querySelector<HTMLButtonElement>('[data-action="refresh"]')
	?.addEventListener("click", () => {
		void loadBonds();
	});

document
	.querySelector<HTMLButtonElement>('[data-view="bonds"]')
	?.addEventListener("click", () => {
		document
			.querySelectorAll(".menu-item")
			.forEach((item) => item.classList.remove("is-active"));
		document
			.querySelector(".menu-item[data-view='bonds']")
			?.classList.add("is-active");
	});

void loadBonds();