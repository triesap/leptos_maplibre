import maplibregl from "https://esm.sh/maplibre-gl@5.13.0";
let next_id = 1;
const maps = new globalThis.Map();
const observers = new globalThis.Map();
const click_cbs = new globalThis.Map();
const load_cbs = new globalThis.Map();
const map_event_cbs = new globalThis.Map();
const layer_event_cbs = new globalThis.Map();
const click_handlers = new globalThis.Map();
const load_handlers = new globalThis.Map();
const map_event_handlers = new globalThis.Map();
const layer_event_handlers = new globalThis.Map();
const layer_key_delimiter = "\u0000";
function log_bridge_error(context, error) {
    console.error(`leptos_maplibre ${context}:`, error);
}
function get_map(handle) {
    return maps.get(handle);
}
function is_nil(value) {
    return value === undefined || value === null;
}
function optional(value) {
    return is_nil(value) ? undefined : value;
}
function to_finite_number(value) {
    if (typeof value === "number" && Number.isFinite(value)) {
        return value;
    }
    if (typeof value === "string" && value.trim() !== "") {
        const parsed = Number(value);
        if (Number.isFinite(parsed)) {
            return parsed;
        }
    }
    return undefined;
}
function to_center_lng(value) {
    const parsed = to_finite_number(value);
    if (parsed === undefined) {
        return 0;
    }
    return parsed < -180 || parsed > 180 ? 0 : parsed;
}
function to_center_lat(value) {
    const parsed = to_finite_number(value);
    if (parsed === undefined) {
        return 0;
    }
    return parsed < -90 || parsed > 90 ? 0 : parsed;
}
function to_control_anchor(anchor) {
    if (is_nil(anchor)) {
        return undefined;
    }
    return anchor.replace("_", "-");
}
function to_bounds(bounds) {
    if (is_nil(bounds) || !Array.isArray(bounds) || bounds.length !== 4) {
        return undefined;
    }
    const west = to_finite_number(bounds[0]);
    const south = to_finite_number(bounds[1]);
    const east = to_finite_number(bounds[2]);
    const north = to_finite_number(bounds[3]);
    if (west === undefined ||
        south === undefined ||
        east === undefined ||
        north === undefined) {
        return undefined;
    }
    return [
        [west, south],
        [east, north],
    ];
}
function to_feature_hit(feature) {
    const properties = {};
    const source_properties = feature.properties ?? {};
    for (const [key, value] of Object.entries(source_properties)) {
        properties[key] = value;
    }
    return {
        layer_id: feature.layer.id,
        properties,
    };
}
function resolve_feature_id(feature_id) {
    if (typeof feature_id === "string" || typeof feature_id === "number") {
        return feature_id;
    }
    if (typeof feature_id === "object" &&
        feature_id !== null &&
        "id" in feature_id) {
        const id = feature_id.id;
        if (typeof id === "string" || typeof id === "number") {
            return id;
        }
    }
    return undefined;
}
function current_view_state(map) {
    const center = map.getCenter();
    return {
        center_lng: center.lng,
        center_lat: center.lat,
        zoom: map.getZoom(),
        bearing: map.getBearing(),
        pitch: map.getPitch(),
    };
}
function emit_map_event(handle, map, kind) {
    const callback = map_event_cbs.get(handle);
    if (callback === undefined) {
        return;
    }
    callback({
        kind,
        view: current_view_state(map),
    });
}
function layer_key(handle, layer_id) {
    return `${handle}${layer_key_delimiter}${layer_id}`;
}
function layer_key_prefix(handle) {
    return `${handle}${layer_key_delimiter}`;
}
function query_layer_features(map, event, layer_id) {
    if (event.features !== undefined) {
        return event.features;
    }
    return map.queryRenderedFeatures(event.point, { layers: [layer_id] });
}
function emit_layer_event(handle, layer_id, map, event, kind) {
    const key = layer_key(handle, layer_id);
    const callback = layer_event_cbs.get(key);
    if (callback === undefined) {
        return;
    }
    const features = query_layer_features(map, event, layer_id).map(to_feature_hit);
    callback({
        kind,
        layer_id,
        lng: event.lngLat.lng,
        lat: event.lngLat.lat,
        screen_x: event.point.x,
        screen_y: event.point.y,
        features,
    });
}
function apply_native_controls(map, controls) {
    if (is_nil(controls)) {
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
export function init_map(container, options) {
    const canvas_context_attributes = is_nil(options.antialias) ? undefined : { antialias: options.antialias };
    const min_zoom = to_finite_number(options.min_zoom);
    const raw_max_zoom = to_finite_number(options.max_zoom);
    const max_zoom = raw_max_zoom !== undefined && min_zoom !== undefined && raw_max_zoom < min_zoom
        ? min_zoom
        : raw_max_zoom;
    if (container.clientWidth <= 0 || container.clientHeight <= 0) {
        if (container.style.width === "" || container.style.width === "100%") {
            container.style.width = "1024px";
        }
        if (container.style.minHeight === "") {
            container.style.minHeight = "400px";
        }
        if (container.style.height === "") {
            container.style.height = "400px";
        }
    }
    const map_options = {
        container,
        style: typeof options.style_url === "string" && options.style_url !== ""
            ? options.style_url
            : "https://demotiles.maplibre.org/style.json",
        center: [to_center_lng(options.center_lng), to_center_lat(options.center_lat)],
        zoom: to_finite_number(options.zoom) ?? 2,
        attributionControl: options.attribution_control !== false,
        centerClampedToGround: false,
        canvasContextAttributes: canvas_context_attributes,
    };
    if (min_zoom !== undefined) {
        map_options.minZoom = min_zoom;
    }
    if (max_zoom !== undefined) {
        map_options.maxZoom = max_zoom;
    }
    const bounds = to_bounds(options.bounds);
    if (bounds !== undefined) {
        map_options.bounds = bounds;
    }
    const pitch = to_finite_number(options.pitch);
    if (pitch !== undefined) {
        map_options.pitch = pitch;
    }
    const bearing = to_finite_number(options.bearing);
    if (bearing !== undefined) {
        map_options.bearing = bearing;
    }
    if (typeof options.interactive === "boolean") {
        map_options.interactive = options.interactive;
    }
    let map;
    try {
        map = new maplibregl.Map(map_options);
    }
    catch (error) {
        log_bridge_error("init_map_options", {
            ...map_options,
            container_width: container.clientWidth,
            container_height: container.clientHeight,
        });
        throw error;
    }
    apply_native_controls(map, options.native_controls);
    const handle = next_id;
    next_id += 1;
    maps.set(handle, map);
    const observer = new ResizeObserver(() => {
        try {
            map.resize();
        }
        catch (error) {
            log_bridge_error("resize_observer", error);
        }
    });
    observer.observe(container);
    observers.set(handle, observer);
    const click_handler = (event) => {
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
export function destroy_map(handle) {
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
    const prefix = layer_key_prefix(handle);
    for (const [key, handlers] of layer_event_handlers.entries()) {
        if (!key.startsWith(prefix)) {
            continue;
        }
        const layer_id = key.slice(prefix.length);
        map.off("click", layer_id, handlers.click);
        map.off("dblclick", layer_id, handlers.dblclick);
        map.off("contextmenu", layer_id, handlers.contextmenu);
        map.off("mouseenter", layer_id, handlers.mouseenter);
        map.off("mousemove", layer_id, handlers.mousemove);
        map.off("mouseleave", layer_id, handlers.mouseleave);
        layer_event_handlers.delete(key);
        layer_event_cbs.delete(key);
    }
    try {
        map.remove();
    }
    catch (error) {
        log_bridge_error("destroy_map", error);
    }
    finally {
        maps.delete(handle);
    }
}
export function resize_map(handle) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    try {
        map.resize();
    }
    catch (error) {
        log_bridge_error("resize_map", error);
    }
}
export function set_style(handle, style_url) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    try {
        map.setStyle(style_url);
    }
    catch (error) {
        log_bridge_error("set_style", error);
    }
}
export function add_geojson_source(handle, source_id, geojson, promote_id) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    try {
        const existing = map.getSource(source_id);
        if (existing !== undefined) {
            if ("setData" in existing) {
                existing.setData(geojson);
                return;
            }
            map.removeSource(source_id);
        }
        const source = {
            type: "geojson",
            data: geojson,
        };
        if (promote_id !== undefined && promote_id !== null) {
            source.promoteId = promote_id;
        }
        map.addSource(source_id, source);
    }
    catch (error) {
        log_bridge_error("add_geojson_source", error);
    }
}
export function update_geojson_source(handle, source_id, geojson) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    try {
        const source = map.getSource(source_id);
        if (source === undefined || !("setData" in source)) {
            return;
        }
        source.setData(geojson);
    }
    catch (error) {
        log_bridge_error("update_geojson_source", error);
    }
}
export function remove_source(handle, source_id) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    try {
        const source = map.getSource(source_id);
        if (source !== undefined) {
            map.removeSource(source_id);
        }
    }
    catch (error) {
        log_bridge_error("remove_source", error);
    }
}
export function add_layer(handle, layer_id, layer_spec, before_id) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    try {
        const normalized_layer = {
            ...layer_spec,
            id: layer_id,
        };
        map.addLayer(normalized_layer, before_id ?? undefined);
    }
    catch (error) {
        log_bridge_error("add_layer", error);
    }
}
export function remove_layer(handle, layer_id) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    try {
        if (map.getLayer(layer_id) !== undefined) {
            map.removeLayer(layer_id);
        }
    }
    catch (error) {
        log_bridge_error("remove_layer", error);
    }
}
export function set_feature_state(handle, source_id, source_layer, feature_id, state) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    try {
        const resolved_id = resolve_feature_id(feature_id);
        if (resolved_id === undefined) {
            return;
        }
        map.setFeatureState({
            source: source_id,
            sourceLayer: source_layer ?? undefined,
            id: resolved_id,
        }, state);
    }
    catch (error) {
        log_bridge_error("set_feature_state", error);
    }
}
export function fly_to(handle, lng, lat, zoom, duration_ms) {
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
    }
    catch (error) {
        log_bridge_error("fly_to", error);
    }
}
export function register_on_click(handle, cb) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    click_cbs.set(handle, cb);
}
export function unregister_on_click(handle) {
    click_cbs.delete(handle);
}
export function register_on_load(handle, cb) {
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
    const load_handler = () => {
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
export function unregister_on_load(handle) {
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
export function register_on_map_events(handle, cb) {
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
    const handlers = {
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
export function unregister_on_map_events(handle) {
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
export function register_on_layer_events(handle, layer_id, cb) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    const key = layer_key(handle, layer_id);
    layer_event_cbs.set(key, cb);
    const previous = layer_event_handlers.get(key);
    if (previous !== undefined) {
        map.off("click", layer_id, previous.click);
        map.off("dblclick", layer_id, previous.dblclick);
        map.off("contextmenu", layer_id, previous.contextmenu);
        map.off("mouseenter", layer_id, previous.mouseenter);
        map.off("mousemove", layer_id, previous.mousemove);
        map.off("mouseleave", layer_id, previous.mouseleave);
        layer_event_handlers.delete(key);
    }
    const handlers = {
        click: (event) => emit_layer_event(handle, layer_id, map, event, "click"),
        dblclick: (event) => emit_layer_event(handle, layer_id, map, event, "double_click"),
        contextmenu: (event) => emit_layer_event(handle, layer_id, map, event, "context_menu"),
        mouseenter: (event) => emit_layer_event(handle, layer_id, map, event, "mouse_enter"),
        mousemove: (event) => emit_layer_event(handle, layer_id, map, event, "mouse_move"),
        mouseleave: (event) => emit_layer_event(handle, layer_id, map, event, "mouse_leave"),
    };
    layer_event_handlers.set(key, handlers);
    map.on("click", layer_id, handlers.click);
    map.on("dblclick", layer_id, handlers.dblclick);
    map.on("contextmenu", layer_id, handlers.contextmenu);
    map.on("mouseenter", layer_id, handlers.mouseenter);
    map.on("mousemove", layer_id, handlers.mousemove);
    map.on("mouseleave", layer_id, handlers.mouseleave);
}
export function unregister_on_layer_events(handle, layer_id) {
    const key = layer_key(handle, layer_id);
    const map = get_map(handle);
    if (map === undefined) {
        layer_event_cbs.delete(key);
        layer_event_handlers.delete(key);
        return;
    }
    const handlers = layer_event_handlers.get(key);
    if (handlers !== undefined) {
        map.off("click", layer_id, handlers.click);
        map.off("dblclick", layer_id, handlers.dblclick);
        map.off("contextmenu", layer_id, handlers.contextmenu);
        map.off("mouseenter", layer_id, handlers.mouseenter);
        map.off("mousemove", layer_id, handlers.mousemove);
        map.off("mouseleave", layer_id, handlers.mouseleave);
        layer_event_handlers.delete(key);
    }
    layer_event_cbs.delete(key);
}
