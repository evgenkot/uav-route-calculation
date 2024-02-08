import { writable } from 'svelte/store';
export interface Camera {
	id: number;
	name: string;
	mass: number;
	fov_x: number;
	resolution_x: number;
	resolution_y: number;
}

export interface Uav {
	id: number;
	name: string;
	max_payload_mass: number;
	flight_duration: number;
	takeoff_speed: number;
	flight_speed: number;
	min_altitude: number;
	max_altitude: number;
}


export const selectedUav = writable<Uav | null>(null);
export const selectedCamera = writable<Camera | null>(null);

export const altitudeValue = writable<number>(0);
export const overlapValue = writable<number>(0);

export enum Algorithm {
	NearestNeighbor = "NearestNeighbor",
	BruteForce = "BruteForce",
}
export const selectedAlgorithm = writable<Algorithm>(Algorithm.NearestNeighbor);

export const utmZone = writable<string>('EPSG:3857');
export const planInMeters = writable<number[][]>([]);

export const routeLength = writable<number>(0);
export const missionDuration = writable<number>(0);
export const photoCount = writable<number>(0);


import Map from 'ol/Map';
import Draw from 'ol/interaction/Draw';
import Modify from 'ol/interaction/Modify';
import Snap from 'ol/interaction/Snap';
import View from 'ol/View';
import { OSM, Vector as VectorSource } from 'ol/source';
import { Tile as TileLayer, Vector as VectorLayer } from 'ol/layer';
import Style from 'ol/style/Style';
import Circle from 'ol/style/Circle';
import Fill from 'ol/style/Fill';
import Stroke from 'ol/style/Stroke';
import type { Geometry } from 'ol/geom';

let viewMap = 'main-map';

export const startPointSource = writable<VectorSource<Geometry>>(new VectorSource());
export const vectorPolySource = writable<VectorSource<Geometry>>(new VectorSource({ wrapX: false }));
export const discretizedAreaLayer = writable<VectorLayer<VectorSource<Geometry>>>(new VectorLayer({}));
export const planLayer = writable<VectorLayer<VectorSource<Geometry>>>(new VectorLayer({}));


const osmLayer = new TileLayer({ source: new OSM() });

const vector = new VectorLayer({
	source: new VectorSource({ wrapX: false })
});

const startPointLayer = new VectorLayer();

let discretizedAreaLayerTmp = new VectorLayer({
	source: new VectorSource(),
	style: new Style({
		image: new Circle({
			radius: 5,
			fill: new Fill({
				color: 'rgba(0, 255, 0, 0.7)'
			})
		})
	})
});

let planLayerTMP = new VectorLayer({
	source: new VectorSource(),
	style: new Style({
		stroke: new Stroke({
			color: 'rgba(0, 0, 255, 0.7)',
			width: 3
		})
	})
});

export const map = writable<Map>(new Map({
	target: viewMap,
	layers: [osmLayer, vector, startPointLayer, discretizedAreaLayerTmp, planLayerTMP],
	view: new View({
		center: [0, 0],
		zoom: 2
	})
}));
export const drawInteraction = writable<Draw>(new Draw({
	source: new VectorSource({ wrapX: false }),
	type: 'Polygon'
}));

export const modifyInteraction = writable<Modify>(new Modify({ source: new VectorSource() }));
export const snapInteraction = writable<Snap>(new Snap({ source: new VectorSource() }));

// Sequencing
export const areaSelected = writable<Boolean>(false);
export const startSelected = writable<Boolean>(false);
export const altitudeSelected = writable<Boolean>(false);
export const areaDiscretized = writable<Boolean>(false);

export const isDrawing = writable<Boolean>(false);

export const discretizationDirection = writable<number>(0);

export const discretizedArea = writable<number[][]>([]);
export const startingPoint = writable<number[] | null>(null);
export const planResult = writable<number[][]>([]);
