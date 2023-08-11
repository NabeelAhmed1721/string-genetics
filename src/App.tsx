import { createSignal, onCleanup } from 'solid-js';
import { Gene, Pool } from './wasm/string_genetics';

export default function App() {
	const [generationCount, setGenerationCount] = createSignal(0);
	const [isConverging, setConverging] = createSignal(false);
	let interval: number = 0;
	let targetInput: HTMLInputElement | undefined;
	let progressContainer: HTMLDivElement | undefined;
	let fitnessContainer: HTMLDivElement | undefined;

	async function handleConvergence() {
		setConverging(false);
		if (!targetInput || !targetInput.value || !progressContainer) return;

		// setProgress([]);

		progressContainer.replaceChildren();
		setGenerationCount(0);

		const target = new Gene(targetInput.value);
		const pool = new Pool(target, 256);

		// for (let x = 0; x < 512; x++) {)
		setConverging(true);

		interval = setInterval(() => {
			if (targetInput && progressContainer && fitnessContainer) {
				pool.naturalSelection();

				const best = pool.getBest();
				const bestDNA = best.toString();

				if (
					progressContainer.lastChild &&
					progressContainer.childNodes.length >= 100
				) {
					progressContainer.removeChild(progressContainer.lastChild);
				}

				fitnessContainer.innerText = `Current fitness: ${best
					.calcFitness(new Gene(targetInput.value))
					.toFixed(3)}`;

				setGenerationCount((prev) => prev + 1);

				const child = document.createElement('div');
				child.innerText = bestDNA;
				progressContainer?.prepend(child);

				if (best.calcFitness(new Gene(targetInput.value)) === 1.0) {
					// break;
					clearInterval(interval);
					setConverging(false);
					pool.free();
				}
			}
		}, 100);
	}

	onCleanup(() => {
		clearInterval(interval);
	});

	return (
		<main class="mx-auto max-w-xl bg-neutral-800 p-8">
			<section>
				<article>
					<h1 class="text-xl font-bold">String Genetics</h1>
					<p class="mt-2">
						String-based genetic algorithm web app demo.
					</p>
					<p class="mt-2 italic">
						Made by:{' '}
						<a
							href="https://pillar.land/"
							class="hover:underline"
							target="_blank"
						>
							Nabeel Ahmed @ Pillarland
						</a>
					</p>
					<hr class="my-8" />
					<p>Specify a target text:</p>
					<input
						class="mt-4 w-full p-4 text-black outline-none read-only:cursor-default"
						type="text"
						placeholder="Enter target text here..."
						ref={targetInput}
						readonly={isConverging()}
					/>
					<button
						class="mt-4 w-full border border-white p-4 hover:bg-neutral-600 disabled:border-neutral-600 disabled:hover:bg-transparent"
						onClick={handleConvergence}
						disabled={isConverging()}
					>
						{isConverging() ? 'Converging...' : 'Begin convergence'}
					</button>

					<p class="mt-4"> Gene progress:</p>
					<div class="flex items-center justify-between">
						<p class="tabular-nums">
							Generation: {generationCount()}
						</p>
						<p class="tabular-nums" ref={fitnessContainer}>
							Current fitness:
						</p>
					</div>
					<div
						class="mt-4 h-48 overflow-auto bg-neutral-600 p-4"
						ref={progressContainer}
					/>
				</article>
			</section>
		</main>
	);
}
