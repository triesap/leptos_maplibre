import maplibregl from "https://esm.sh/maplibre-gl@5.13.0";
let next_id = 1;
let next_marker_id = 1;
let next_popup_id = 1;
const maps = new globalThis.Map();
const observers = new globalThis.Map();
const markers = new globalThis.Map();
const marker_maps = new globalThis.Map();
const map_markers = new globalThis.Map();
const marker_drag_event_cbs = new globalThis.Map();
const marker_drag_event_handlers = new globalThis.Map();
const popups = new globalThis.Map();
const popup_maps = new globalThis.Map();
const map_popups = new globalThis.Map();
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
function to_boolean(value) {
    return typeof value === "boolean" ? value : undefined;
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
function emit_map_event(handle, map, kind, extras) {
    const callback = map_event_cbs.get(handle);
    if (callback === undefined) {
        return;
    }
    callback({
        kind,
        view: current_view_state(map),
        ...(extras ?? {}),
    });
}
function layer_key(handle, layer_id) {
    return `${handle}${layer_key_delimiter}${layer_id}`;
}
function layer_key_prefix(handle) {
    return `${handle}${layer_key_delimiter}`;
}
function track_marker(handle, marker_handle) {
    const existing = map_markers.get(handle);
    if (existing !== undefined) {
        existing.add(marker_handle);
    }
    else {
        map_markers.set(handle, new globalThis.Set([marker_handle]));
    }
    marker_maps.set(marker_handle, handle);
}
function untrack_marker(marker_handle) {
    const handle = marker_maps.get(marker_handle);
    if (handle === undefined) {
        return;
    }
    marker_maps.delete(marker_handle);
    const marker_set = map_markers.get(handle);
    if (marker_set === undefined) {
        return;
    }
    marker_set.delete(marker_handle);
    if (marker_set.size === 0) {
        map_markers.delete(handle);
    }
}
function detach_marker_drag_events(marker_handle) {
    const marker = markers.get(marker_handle);
    const handlers = marker_drag_event_handlers.get(marker_handle);
    if (marker !== undefined && handlers !== undefined) {
        marker.off("dragstart", handlers.dragstart);
        marker.off("drag", handlers.drag);
        marker.off("dragend", handlers.dragend);
    }
    marker_drag_event_handlers.delete(marker_handle);
    marker_drag_event_cbs.delete(marker_handle);
}
function track_popup(handle, popup_handle) {
    const existing = map_popups.get(handle);
    if (existing !== undefined) {
        existing.add(popup_handle);
    }
    else {
        map_popups.set(handle, new globalThis.Set([popup_handle]));
    }
    popup_maps.set(popup_handle, handle);
}
function untrack_popup(popup_handle) {
    const handle = popup_maps.get(popup_handle);
    if (handle === undefined) {
        return;
    }
    popup_maps.delete(popup_handle);
    const popup_set = map_popups.get(handle);
    if (popup_set === undefined) {
        return;
    }
    popup_set.delete(popup_handle);
    if (popup_set.size === 0) {
        map_popups.delete(handle);
    }
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
    const min_pitch = to_finite_number(options.min_pitch);
    const raw_max_pitch = to_finite_number(options.max_pitch);
    const max_pitch = raw_max_pitch !== undefined && min_pitch !== undefined && raw_max_pitch < min_pitch
        ? min_pitch
        : raw_max_pitch;
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
    if (min_pitch !== undefined) {
        map_options.minPitch = min_pitch;
    }
    if (max_pitch !== undefined) {
        map_options.maxPitch = max_pitch;
    }
    const bounds = to_bounds(options.bounds);
    if (bounds !== undefined) {
        map_options.bounds = bounds;
    }
    const max_bounds = to_bounds(options.max_bounds);
    if (max_bounds !== undefined) {
        map_options.maxBounds = max_bounds;
    }
    const pitch = to_finite_number(options.pitch);
    if (pitch !== undefined) {
        map_options.pitch = pitch;
    }
    const bearing = to_finite_number(options.bearing);
    if (bearing !== undefined) {
        map_options.bearing = bearing;
    }
    const bearing_snap = to_finite_number(options.bearing_snap);
    if (bearing_snap !== undefined) {
        map_options.bearingSnap = bearing_snap;
    }
    if (typeof options.projection === "string" && options.projection !== "") {
        map_options.projection = { type: options.projection };
    }
    const render_world_copies = to_boolean(options.render_world_copies);
    if (render_world_copies !== undefined) {
        map_options.renderWorldCopies = render_world_copies;
    }
    const drag_pan = to_boolean(options.drag_pan);
    if (drag_pan !== undefined) {
        map_options.dragPan = drag_pan;
    }
    const drag_rotate = to_boolean(options.drag_rotate);
    if (drag_rotate !== undefined) {
        map_options.dragRotate = drag_rotate;
    }
    const pitch_with_rotate = to_boolean(options.pitch_with_rotate);
    if (pitch_with_rotate !== undefined) {
        map_options.pitchWithRotate = pitch_with_rotate;
    }
    const interactive = to_boolean(options.interactive);
    if (interactive !== undefined) {
        map_options.interactive = interactive;
    }
    const cooperative_gestures = to_boolean(options.cooperative_gestures);
    if (cooperative_gestures !== undefined) {
        map_options.cooperativeGestures = cooperative_gestures;
    }
    const preserve_drawing_buffer = to_boolean(options.preserve_drawing_buffer);
    if (preserve_drawing_buffer !== undefined) {
        map_options.preserveDrawingBuffer = preserve_drawing_buffer;
    }
    const around_center = to_boolean(options.around_center);
    if (around_center !== undefined) {
        map_options.aroundCenter = around_center;
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
    const zoom_on_double_click = to_boolean(options.zoom_on_double_click);
    if (zoom_on_double_click === true) {
        map.doubleClickZoom.enable();
    }
    else if (zoom_on_double_click === false) {
        map.doubleClickZoom.disable();
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
        map.off("movestart", map_events.movestart);
        map.off("move", map_events.move);
        map.off("moveend", map_events.moveend);
        map.off("zoomstart", map_events.zoomstart);
        map.off("zoom", map_events.zoom);
        map.off("zoomend", map_events.zoomend);
        map.off("idle", map_events.idle);
        map.off("resize", map_events.resize);
        map.off("render", map_events.render);
        map.off("style.load", map_events.styleload);
        map.off("styledata", map_events.styledata);
        map.off("sourcedata", map_events.sourcedata);
        map.off("data", map_events.data);
        map.off("error", map_events.error);
        map_event_handlers.delete(handle);
    }
    click_cbs.delete(handle);
    load_cbs.delete(handle);
    map_event_cbs.delete(handle);
    const marker_set = map_markers.get(handle);
    if (marker_set !== undefined) {
        for (const marker_handle of marker_set) {
            const marker = markers.get(marker_handle);
            if (marker === undefined) {
                continue;
            }
            try {
                detach_marker_drag_events(marker_handle);
                marker.remove();
            }
            catch (error) {
                log_bridge_error("destroy_map_marker", error);
            }
            markers.delete(marker_handle);
            marker_maps.delete(marker_handle);
        }
        map_markers.delete(handle);
    }
    const popup_set = map_popups.get(handle);
    if (popup_set !== undefined) {
        for (const popup_handle of popup_set) {
            const popup = popups.get(popup_handle);
            if (popup === undefined) {
                continue;
            }
            try {
                popup.remove();
            }
            catch (error) {
                log_bridge_error("destroy_map_popup", error);
            }
            popups.delete(popup_handle);
            popup_maps.delete(popup_handle);
        }
        map_popups.delete(handle);
    }
    const prefix = layer_key_prefix(handle);
    for (const [key, handlers] of layer_event_handlers.entries()) {
        if (!key.startsWith(prefix)) {
            continue;
        }
        const layer_id = key.slice(prefix.length);
        map.off("click", layer_id, handlers.click);
        map.off("dblclick", layer_id, handlers.dblclick);
        map.off("contextmenu", layer_id, handlers.contextmenu);
        map.off("mousedown", layer_id, handlers.mousedown);
        map.off("mouseup", layer_id, handlers.mouseup);
        map.off("mouseover", layer_id, handlers.mouseover);
        map.off("mouseout", layer_id, handlers.mouseout);
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
export function add_source(handle, source_id, source_spec) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    if (typeof source_spec !== "object" || source_spec === null) {
        return;
    }
    const source_type = source_spec.type;
    if (typeof source_type !== "string" || source_type === "") {
        return;
    }
    try {
        if (map.getSource(source_id) !== undefined) {
            map.removeSource(source_id);
        }
        map.addSource(source_id, source_spec);
    }
    catch (error) {
        log_bridge_error("add_source", error);
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
export function set_layout_property(handle, layer_id, property_name, value) {
    const map = get_map(handle);
    if (map === undefined || map.getLayer(layer_id) === undefined) {
        return;
    }
    if (typeof property_name !== "string" || property_name === "") {
        return;
    }
    try {
        map.setLayoutProperty(layer_id, property_name, value);
    }
    catch (error) {
        log_bridge_error("set_layout_property", error);
    }
}
export function set_paint_property(handle, layer_id, property_name, value) {
    const map = get_map(handle);
    if (map === undefined || map.getLayer(layer_id) === undefined) {
        return;
    }
    if (typeof property_name !== "string" || property_name === "") {
        return;
    }
    try {
        map.setPaintProperty(layer_id, property_name, value);
    }
    catch (error) {
        log_bridge_error("set_paint_property", error);
    }
}
export function set_filter(handle, layer_id, filter) {
    const map = get_map(handle);
    if (map === undefined || map.getLayer(layer_id) === undefined) {
        return;
    }
    try {
        map.setFilter(layer_id, filter ?? null);
    }
    catch (error) {
        log_bridge_error("set_filter", error);
    }
}
export function set_layer_zoom_range(handle, layer_id, min_zoom, max_zoom) {
    const map = get_map(handle);
    const layer = map?.getLayer(layer_id);
    if (map === undefined || layer === undefined) {
        return;
    }
    const resolved_min_zoom = to_finite_number(min_zoom) ?? layer.minzoom ?? 0;
    const raw_max_zoom = to_finite_number(max_zoom) ?? layer.maxzoom ?? 24;
    const resolved_max_zoom = raw_max_zoom < resolved_min_zoom ? resolved_min_zoom : raw_max_zoom;
    try {
        map.setLayerZoomRange(layer_id, resolved_min_zoom, resolved_max_zoom);
    }
    catch (error) {
        log_bridge_error("set_layer_zoom_range", error);
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
export function jump_to(handle, lng, lat, zoom, bearing, pitch) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    try {
        map.jumpTo({
            center: [lng, lat],
            zoom: zoom ?? map.getZoom(),
            bearing: bearing ?? map.getBearing(),
            pitch: pitch ?? map.getPitch(),
        });
    }
    catch (error) {
        log_bridge_error("jump_to", error);
    }
}
export function ease_to(handle, lng, lat, zoom, bearing, pitch, duration_ms) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    try {
        map.easeTo({
            center: [lng, lat],
            zoom: zoom ?? map.getZoom(),
            bearing: bearing ?? map.getBearing(),
            pitch: pitch ?? map.getPitch(),
            duration: duration_ms ?? undefined,
        });
    }
    catch (error) {
        log_bridge_error("ease_to", error);
    }
}
export function fit_bounds(handle, west, south, east, north, padding, duration_ms, max_zoom) {
    const map = get_map(handle);
    if (map === undefined) {
        return;
    }
    const safe_west = to_finite_number(west);
    const safe_south = to_finite_number(south);
    const safe_east = to_finite_number(east);
    const safe_north = to_finite_number(north);
    if (safe_west === undefined ||
        safe_south === undefined ||
        safe_east === undefined ||
        safe_north === undefined) {
        return;
    }
    try {
        map.fitBounds([
            [safe_west, safe_south],
            [safe_east, safe_north],
        ], {
            padding: to_finite_number(padding),
            duration: duration_ms ?? undefined,
            maxZoom: to_finite_number(max_zoom),
        });
    }
    catch (error) {
        log_bridge_error("fit_bounds", error);
    }
}
export function create_marker(
    handle,
    lng,
    lat,
    draggable,
    anchor,
    offset_x,
    offset_y,
    rotation,
) {
    const map = get_map(handle);
    if (map === undefined) {
        return 0;
    }
    try {
        const marker_options = {
            draggable: draggable === true,
        };
        const marker_anchor = to_control_anchor(anchor);
        if (marker_anchor !== undefined) {
            marker_options.anchor = marker_anchor;
        }
        const resolved_offset_x = to_finite_number(offset_x);
        const resolved_offset_y = to_finite_number(offset_y);
        if (resolved_offset_x !== undefined && resolved_offset_y !== undefined) {
            marker_options.offset = [resolved_offset_x, resolved_offset_y];
        }
        const marker_rotation = to_finite_number(rotation);
        if (marker_rotation !== undefined) {
            marker_options.rotation = marker_rotation;
        }
        const marker = new maplibregl.Marker(marker_options)
            .setLngLat([lng, lat])
            .addTo(map);
        const marker_handle = next_marker_id;
        next_marker_id += 1;
        markers.set(marker_handle, marker);
        track_marker(handle, marker_handle);
        return marker_handle;
    }
    catch (error) {
        log_bridge_error("create_marker", error);
        return 0;
    }
}
export function update_marker(
    marker_handle,
    lng,
    lat,
    draggable,
    anchor,
    offset_x,
    offset_y,
    rotation,
) {
    const marker = markers.get(marker_handle);
    if (marker === undefined) {
        return;
    }
    try {
        marker.setLngLat([lng, lat]);
        marker.setDraggable(draggable === true);
        const marker_anchor = to_control_anchor(anchor);
        if (marker_anchor !== undefined && typeof marker.setAnchor === "function") {
            marker.setAnchor(marker_anchor);
        }
        const resolved_offset_x = to_finite_number(offset_x);
        const resolved_offset_y = to_finite_number(offset_y);
        if (resolved_offset_x !== undefined &&
            resolved_offset_y !== undefined &&
            typeof marker.setOffset === "function") {
            marker.setOffset([resolved_offset_x, resolved_offset_y]);
        }
        const marker_rotation = to_finite_number(rotation);
        if (marker_rotation !== undefined && typeof marker.setRotation === "function") {
            marker.setRotation(marker_rotation);
        }
    }
    catch (error) {
        log_bridge_error("update_marker", error);
    }
}
export function remove_marker(marker_handle) {
    const marker = markers.get(marker_handle);
    if (marker === undefined) {
        return;
    }
    try {
        marker.remove();
    }
    catch (error) {
        log_bridge_error("remove_marker", error);
    }
    finally {
        detach_marker_drag_events(marker_handle);
        markers.delete(marker_handle);
        untrack_marker(marker_handle);
    }
}
function emit_marker_drag_event(marker_handle, kind) {
    const marker = markers.get(marker_handle);
    const callback = marker_drag_event_cbs.get(marker_handle);
    if (marker === undefined || callback === undefined) {
        return;
    }
    const lng_lat = marker.getLngLat();
    callback({
        kind,
        lng: lng_lat.lng,
        lat: lng_lat.lat,
    });
}
export function register_on_marker_drag_events(marker_handle, cb) {
    const marker = markers.get(marker_handle);
    if (marker === undefined) {
        return;
    }
    marker_drag_event_cbs.set(marker_handle, cb);
    detach_marker_drag_events(marker_handle);
    marker_drag_event_cbs.set(marker_handle, cb);
    const handlers = {
        dragstart: () => emit_marker_drag_event(marker_handle, "drag_start"),
        drag: () => emit_marker_drag_event(marker_handle, "drag"),
        dragend: () => emit_marker_drag_event(marker_handle, "drag_end"),
    };
    marker_drag_event_handlers.set(marker_handle, handlers);
    marker.on("dragstart", handlers.dragstart);
    marker.on("drag", handlers.drag);
    marker.on("dragend", handlers.dragend);
}
export function unregister_on_marker_drag_events(marker_handle) {
    detach_marker_drag_events(marker_handle);
}
export function create_popup(handle, lng, lat, html, close_button, close_on_click) {
    const map = get_map(handle);
    if (map === undefined) {
        return 0;
    }
    try {
        const popup = new maplibregl.Popup({
            closeButton: close_button === true,
            closeOnClick: close_on_click === true,
        })
            .setLngLat([lng, lat])
            .setHTML(html)
            .addTo(map);
        const popup_handle = next_popup_id;
        next_popup_id += 1;
        popups.set(popup_handle, popup);
        track_popup(handle, popup_handle);
        return popup_handle;
    }
    catch (error) {
        log_bridge_error("create_popup", error);
        return 0;
    }
}
export function update_popup(popup_handle, lng, lat, html) {
    const popup = popups.get(popup_handle);
    if (popup === undefined) {
        return;
    }
    try {
        popup.setLngLat([lng, lat]);
        popup.setHTML(html);
    }
    catch (error) {
        log_bridge_error("update_popup", error);
    }
}
export function remove_popup(popup_handle) {
    const popup = popups.get(popup_handle);
    if (popup === undefined) {
        return;
    }
    try {
        popup.remove();
    }
    catch (error) {
        log_bridge_error("remove_popup", error);
    }
    finally {
        popups.delete(popup_handle);
        untrack_popup(popup_handle);
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
        map.off("movestart", previous.movestart);
        map.off("move", previous.move);
        map.off("moveend", previous.moveend);
        map.off("zoomstart", previous.zoomstart);
        map.off("zoom", previous.zoom);
        map.off("zoomend", previous.zoomend);
        map.off("idle", previous.idle);
        map.off("resize", previous.resize);
        map.off("render", previous.render);
        map.off("style.load", previous.styleload);
        map.off("styledata", previous.styledata);
        map.off("sourcedata", previous.sourcedata);
        map.off("data", previous.data);
        map.off("error", previous.error);
        map_event_handlers.delete(handle);
    }
    const handlers = {
        movestart: () => emit_map_event(handle, map, "move_start"),
        move: () => emit_map_event(handle, map, "move"),
        moveend: () => emit_map_event(handle, map, "move_end"),
        zoomstart: () => emit_map_event(handle, map, "zoom_start"),
        zoom: () => emit_map_event(handle, map, "zoom"),
        zoomend: () => emit_map_event(handle, map, "zoom_end"),
        idle: () => emit_map_event(handle, map, "idle"),
        resize: () => emit_map_event(handle, map, "resize"),
        render: () => emit_map_event(handle, map, "render"),
        styleload: () => emit_map_event(handle, map, "style_load"),
        styledata: () => emit_map_event(handle, map, "style_data"),
        sourcedata: () => emit_map_event(handle, map, "source_data"),
        data: () => emit_map_event(handle, map, "data"),
        error: (event) => emit_map_event(handle, map, "error", {
            message: typeof event?.error?.message === "string"
                ? event.error.message
                : typeof event?.error === "string"
                    ? event.error
                    : undefined,
        }),
    };
    map_event_handlers.set(handle, handlers);
    map.on("movestart", handlers.movestart);
    map.on("move", handlers.move);
    map.on("moveend", handlers.moveend);
    map.on("zoomstart", handlers.zoomstart);
    map.on("zoom", handlers.zoom);
    map.on("zoomend", handlers.zoomend);
    map.on("idle", handlers.idle);
    map.on("resize", handlers.resize);
    map.on("render", handlers.render);
    map.on("style.load", handlers.styleload);
    map.on("styledata", handlers.styledata);
    map.on("sourcedata", handlers.sourcedata);
    map.on("data", handlers.data);
    map.on("error", handlers.error);
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
        map.off("movestart", handlers.movestart);
        map.off("move", handlers.move);
        map.off("moveend", handlers.moveend);
        map.off("zoomstart", handlers.zoomstart);
        map.off("zoom", handlers.zoom);
        map.off("zoomend", handlers.zoomend);
        map.off("idle", handlers.idle);
        map.off("resize", handlers.resize);
        map.off("render", handlers.render);
        map.off("style.load", handlers.styleload);
        map.off("styledata", handlers.styledata);
        map.off("sourcedata", handlers.sourcedata);
        map.off("data", handlers.data);
        map.off("error", handlers.error);
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
        map.off("mousedown", layer_id, previous.mousedown);
        map.off("mouseup", layer_id, previous.mouseup);
        map.off("mouseover", layer_id, previous.mouseover);
        map.off("mouseout", layer_id, previous.mouseout);
        map.off("mouseenter", layer_id, previous.mouseenter);
        map.off("mousemove", layer_id, previous.mousemove);
        map.off("mouseleave", layer_id, previous.mouseleave);
        layer_event_handlers.delete(key);
    }
    const handlers = {
        click: (event) => emit_layer_event(handle, layer_id, map, event, "click"),
        dblclick: (event) => emit_layer_event(handle, layer_id, map, event, "double_click"),
        contextmenu: (event) => emit_layer_event(handle, layer_id, map, event, "context_menu"),
        mousedown: (event) => emit_layer_event(handle, layer_id, map, event, "mouse_down"),
        mouseup: (event) => emit_layer_event(handle, layer_id, map, event, "mouse_up"),
        mouseover: (event) => emit_layer_event(handle, layer_id, map, event, "mouse_over"),
        mouseout: (event) => emit_layer_event(handle, layer_id, map, event, "mouse_out"),
        mouseenter: (event) => emit_layer_event(handle, layer_id, map, event, "mouse_enter"),
        mousemove: (event) => emit_layer_event(handle, layer_id, map, event, "mouse_move"),
        mouseleave: (event) => emit_layer_event(handle, layer_id, map, event, "mouse_leave"),
    };
    layer_event_handlers.set(key, handlers);
    map.on("click", layer_id, handlers.click);
    map.on("dblclick", layer_id, handlers.dblclick);
    map.on("contextmenu", layer_id, handlers.contextmenu);
    map.on("mousedown", layer_id, handlers.mousedown);
    map.on("mouseup", layer_id, handlers.mouseup);
    map.on("mouseover", layer_id, handlers.mouseover);
    map.on("mouseout", layer_id, handlers.mouseout);
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
        map.off("mousedown", layer_id, handlers.mousedown);
        map.off("mouseup", layer_id, handlers.mouseup);
        map.off("mouseover", layer_id, handlers.mouseover);
        map.off("mouseout", layer_id, handlers.mouseout);
        map.off("mouseenter", layer_id, handlers.mouseenter);
        map.off("mousemove", layer_id, handlers.mousemove);
        map.off("mouseleave", layer_id, handlers.mouseleave);
        layer_event_handlers.delete(key);
    }
    layer_event_cbs.delete(key);
}
