import maplibregl, {
  type ControlPosition,
  type GeoJSONSource,
  type GeoJSONSourceSpecification,
  type LayerSpecification,
  type LngLatBoundsLike,
  type Map as MaplibreMap,
  type MapGeoJSONFeature,
  type MapLayerMouseEvent,
} from "maplibre-gl";

type JsonObject = Record<string, unknown>;

type ControlAnchor = "top_left" | "top_right" | "bottom_left" | "bottom_right";

interface NativeControlOptions {
  navigation?: ControlAnchor;
  scale?: ControlAnchor;
  fullscreen?: ControlAnchor;
  geolocate?: ControlAnchor;
  attribution?: ControlAnchor;
}

interface MapInitOptions {
  style_url: string;
  center_lng: number;
  center_lat: number;
  zoom: number;
  min_zoom?: number;
  max_zoom?: number;
  bounds?: [number, number, number, number];
  pitch?: number;
  bearing?: number;
  interactive?: boolean;
  attribution_control?: boolean;
  antialias?: boolean;
  native_controls?: NativeControlOptions;
}

interface FeatureHit {
  layer_id: string;
  properties: JsonObject;
}

interface MapClickPayload {
  lng: number;
  lat: number;
  screen_x: number;
  screen_y: number;
  features: FeatureHit[];
}

type MapEventKind = "move" | "zoom" | "idle" | "style_data" | "data";

interface MapViewStatePayload {
  center_lng: number;
  center_lat: number;
  zoom: number;
  bearing: number;
  pitch: number;
}

interface MapEventPayload {
  kind: MapEventKind;
  view: MapViewStatePayload;
}

type ClickCallback = (payload: MapClickPayload) => void;
type LoadCallback = () => void;
type MapEventCallback = (payload: MapEventPayload) => void;

interface MapEventHandlers {
  move: () => void;
  zoom: () => void;
  idle: () => void;
  styledata: () => void;
  data: () => void;
}

let next_id = 1;
const maps = new globalThis.Map<number, MaplibreMap>();
const observers = new globalThis.Map<number, ResizeObserver>();
const click_cbs = new globalThis.Map<number, ClickCallback>();
const load_cbs = new globalThis.Map<number, LoadCallback>();
const map_event_cbs = new globalThis.Map<number, MapEventCallback>();
const click_handlers = new globalThis.Map<number, (event: MapLayerMouseEvent) => void>();
const load_handlers = new globalThis.Map<number, () => void>();
const map_event_handlers = new globalThis.Map<number, MapEventHandlers>();

function log_bridge_error(context: string, error: unknown): void {
  console.error(`leptos_maplibre ${context}:`, error);
}

function get_map(handle: number): MaplibreMap | undefined {
  return maps.get(handle);
}

function to_control_anchor(anchor: ControlAnchor | undefined): ControlPosition | undefined {
  if (anchor === undefined) {
    return undefined;
  }
  return anchor.replace("_", "-") as ControlPosition;
}

function to_bounds(bounds: [number, number, number, number] | undefined): LngLatBoundsLike | undefined {
  if (bounds === undefined) {
    return undefined;
  }
  return [
    [bounds[0], bounds[1]],
    [bounds[2], bounds[3]],
  ];
}

function to_feature_hit(feature: MapGeoJSONFeature): FeatureHit {
  const properties: JsonObject = {};
  const source_properties = feature.properties ?? {};
  for (const [key, value] of Object.entries(source_properties)) {
    properties[key] = value;
  }
  return {
    layer_id: feature.layer.id,
    properties,
  };
}

function resolve_feature_id(feature_id: unknown): string | number | undefined {
  if (typeof feature_id === "string" || typeof feature_id === "number") {
    return feature_id;
  }
  if (
    typeof feature_id === "object" &&
    feature_id !== null &&
    "id" in feature_id
  ) {
    const id = (feature_id as { id: unknown }).id;
    if (typeof id === "string" || typeof id === "number") {
      return id;
    }
  }
  return undefined;
}

function current_view_state(map: MaplibreMap): MapViewStatePayload {
  const center = map.getCenter();
  return {
    center_lng: center.lng,
    center_lat: center.lat,
    zoom: map.getZoom(),
    bearing: map.getBearing(),
    pitch: map.getPitch(),
  };
}

function emit_map_event(handle: number, map: MaplibreMap, kind: MapEventKind): void {
  const callback = map_event_cbs.get(handle);
  if (callback === undefined) {
    return;
  }
  callback({
    kind,
    view: current_view_state(map),
  });
}

function apply_native_controls(map: MaplibreMap, controls: NativeControlOptions | undefined): void {
  if (controls === undefined) {
    return;
  }

  const navigation_anchor = to_control_anchor(controls.navigation);
  if (navigation_anchor !== undefined) {
    map.addControl(new maplibregl.NavigationControl(), navigation_anchor);
  }

  const scale_anchor = to_control_anchor(controls.scale);
  if (scale_anchor !== undefined) {
    map.addControl(new maplibregl.ScaleControl(), scale_anchor);
  }

  const fullscreen_anchor = to_control_anchor(controls.fullscreen);
  if (fullscreen_anchor !== undefined) {
    map.addControl(new maplibregl.FullscreenControl(), fullscreen_anchor);
  }

  const geolocate_anchor = to_control_anchor(controls.geolocate);
  if (geolocate_anchor !== undefined) {
    map.addControl(new maplibregl.GeolocateControl({}), geolocate_anchor);
  }

  const attribution_anchor = to_control_anchor(controls.attribution);
  if (attribution_anchor !== undefined) {
    map.addControl(new maplibregl.AttributionControl(), attribution_anchor);
  }
}

export function init_map(container: HTMLElement, options: MapInitOptions): number {
  const canvas_context_attributes =
    options.antialias === undefined ? undefined : { antialias: options.antialias };

  const map = new maplibregl.Map({
    container,
    style: options.style_url,
    center: [options.center_lng, options.center_lat],
    zoom: options.zoom,
    minZoom: options.min_zoom,
    maxZoom: options.max_zoom,
    bounds: to_bounds(options.bounds),
    pitch: options.pitch,
    bearing: options.bearing,
    interactive: options.interactive,
    attributionControl: options.attribution_control ? {} : false,
    canvasContextAttributes: canvas_context_attributes,
  });

  apply_native_controls(map, options.native_controls);

  const handle = next_id;
  next_id += 1;

  maps.set(handle, map);

  const observer = new ResizeObserver(() => {
    try {
      map.resize();
    } catch (error) {
      log_bridge_error("resize_observer", error);
    }
  });
  observer.observe(container);
  observers.set(handle, observer);

  const click_handler = (event: MapLayerMouseEvent): void => {
    const click_cb = click_cbs.get(handle);
    if (click_cb === undefined) {
      return;
    }
    const features = map.queryRenderedFeatures(event.point).map(to_feature_hit);
    click_cb({
      lng: event.lngLat.lng,
      lat: event.lngLat.lat,
      screen_x: event.point.x,
      screen_y: event.point.y,
      features,
    });
  };

  map.on("click", click_handler);
  click_handlers.set(handle, click_handler);

  return handle;
}

export function destroy_map(handle: number): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }

  const observer = observers.get(handle);
  if (observer !== undefined) {
    observer.disconnect();
    observers.delete(handle);
  }

  const click_handler = click_handlers.get(handle);
  if (click_handler !== undefined) {
    map.off("click", click_handler);
    click_handlers.delete(handle);
  }

  const load_handler = load_handlers.get(handle);
  if (load_handler !== undefined) {
    map.off("load", load_handler);
    load_handlers.delete(handle);
  }

  const map_events = map_event_handlers.get(handle);
  if (map_events !== undefined) {
    map.off("move", map_events.move);
    map.off("zoom", map_events.zoom);
    map.off("idle", map_events.idle);
    map.off("styledata", map_events.styledata);
    map.off("data", map_events.data);
    map_event_handlers.delete(handle);
  }

  click_cbs.delete(handle);
  load_cbs.delete(handle);
  map_event_cbs.delete(handle);

  try {
    map.remove();
  } catch (error) {
    log_bridge_error("destroy_map", error);
  } finally {
    maps.delete(handle);
  }
}

export function resize_map(handle: number): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }
  try {
    map.resize();
  } catch (error) {
    log_bridge_error("resize_map", error);
  }
}

export function set_style(handle: number, style_url: string): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }
  try {
    map.setStyle(style_url);
  } catch (error) {
    log_bridge_error("set_style", error);
  }
}

export function add_geojson_source(
  handle: number,
  source_id: string,
  geojson: GeoJSON.GeoJSON,
  promote_id?: string | null,
): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }

  try {
    const existing = map.getSource(source_id);
    if (existing !== undefined) {
      if ("setData" in existing) {
        (existing as GeoJSONSource).setData(geojson);
        return;
      }
      map.removeSource(source_id);
    }

    const source: GeoJSONSourceSpecification = {
      type: "geojson",
      data: geojson,
    };

    if (promote_id !== undefined && promote_id !== null) {
      source.promoteId = promote_id;
    }

    map.addSource(source_id, source);
  } catch (error) {
    log_bridge_error("add_geojson_source", error);
  }
}

export function update_geojson_source(
  handle: number,
  source_id: string,
  geojson: GeoJSON.GeoJSON,
): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }

  try {
    const source = map.getSource(source_id);
    if (source === undefined || !("setData" in source)) {
      return;
    }
    (source as GeoJSONSource).setData(geojson);
  } catch (error) {
    log_bridge_error("update_geojson_source", error);
  }
}

export function remove_source(handle: number, source_id: string): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }

  try {
    const source = map.getSource(source_id);
    if (source !== undefined) {
      map.removeSource(source_id);
    }
  } catch (error) {
    log_bridge_error("remove_source", error);
  }
}

export function add_layer(
  handle: number,
  layer_id: string,
  layer_spec: LayerSpecification,
  before_id?: string | null,
): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }

  try {
    const normalized_layer: LayerSpecification = {
      ...layer_spec,
      id: layer_id,
    };
    map.addLayer(normalized_layer, before_id ?? undefined);
  } catch (error) {
    log_bridge_error("add_layer", error);
  }
}

export function remove_layer(handle: number, layer_id: string): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }

  try {
    if (map.getLayer(layer_id) !== undefined) {
      map.removeLayer(layer_id);
    }
  } catch (error) {
    log_bridge_error("remove_layer", error);
  }
}

export function set_feature_state(
  handle: number,
  source_id: string,
  source_layer: string | null,
  feature_id: unknown,
  state: JsonObject,
): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }

  try {
    const resolved_id = resolve_feature_id(feature_id);
    if (resolved_id === undefined) {
      return;
    }
    map.setFeatureState(
      {
        source: source_id,
        sourceLayer: source_layer ?? undefined,
        id: resolved_id,
      },
      state,
    );
  } catch (error) {
    log_bridge_error("set_feature_state", error);
  }
}

export function fly_to(
  handle: number,
  lng: number,
  lat: number,
  zoom?: number | null,
  duration_ms?: number | null,
): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }

  try {
    map.flyTo({
      center: [lng, lat],
      zoom: zoom ?? map.getZoom(),
      duration: duration_ms ?? undefined,
    });
  } catch (error) {
    log_bridge_error("fly_to", error);
  }
}

export function register_on_click(handle: number, cb: ClickCallback): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }
  click_cbs.set(handle, cb);
}

export function unregister_on_click(handle: number): void {
  click_cbs.delete(handle);
}

export function register_on_load(handle: number, cb: LoadCallback): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }

  load_cbs.set(handle, cb);

  const previous_handler = load_handlers.get(handle);
  if (previous_handler !== undefined) {
    map.off("load", previous_handler);
    load_handlers.delete(handle);
  }

  if (map.isStyleLoaded()) {
    cb();
    return;
  }

  const load_handler = (): void => {
    const callback = load_cbs.get(handle);
    if (callback !== undefined) {
      callback();
    }
    map.off("load", load_handler);
    load_handlers.delete(handle);
  };

  load_handlers.set(handle, load_handler);
  map.on("load", load_handler);
}

export function unregister_on_load(handle: number): void {
  const map = get_map(handle);
  if (map === undefined) {
    load_cbs.delete(handle);
    load_handlers.delete(handle);
    return;
  }

  const load_handler = load_handlers.get(handle);
  if (load_handler !== undefined) {
    map.off("load", load_handler);
    load_handlers.delete(handle);
  }

  load_cbs.delete(handle);
}

export function register_on_map_events(handle: number, cb: MapEventCallback): void {
  const map = get_map(handle);
  if (map === undefined) {
    return;
  }

  map_event_cbs.set(handle, cb);

  const previous = map_event_handlers.get(handle);
  if (previous !== undefined) {
    map.off("move", previous.move);
    map.off("zoom", previous.zoom);
    map.off("idle", previous.idle);
    map.off("styledata", previous.styledata);
    map.off("data", previous.data);
    map_event_handlers.delete(handle);
  }

  const handlers: MapEventHandlers = {
    move: () => emit_map_event(handle, map, "move"),
    zoom: () => emit_map_event(handle, map, "zoom"),
    idle: () => emit_map_event(handle, map, "idle"),
    styledata: () => emit_map_event(handle, map, "style_data"),
    data: () => emit_map_event(handle, map, "data"),
  };

  map_event_handlers.set(handle, handlers);
  map.on("move", handlers.move);
  map.on("zoom", handlers.zoom);
  map.on("idle", handlers.idle);
  map.on("styledata", handlers.styledata);
  map.on("data", handlers.data);
}

export function unregister_on_map_events(handle: number): void {
  const map = get_map(handle);
  if (map === undefined) {
    map_event_cbs.delete(handle);
    map_event_handlers.delete(handle);
    return;
  }

  const handlers = map_event_handlers.get(handle);
  if (handlers !== undefined) {
    map.off("move", handlers.move);
    map.off("zoom", handlers.zoom);
    map.off("idle", handlers.idle);
    map.off("styledata", handlers.styledata);
    map.off("data", handlers.data);
    map_event_handlers.delete(handle);
  }

  map_event_cbs.delete(handle);
}
