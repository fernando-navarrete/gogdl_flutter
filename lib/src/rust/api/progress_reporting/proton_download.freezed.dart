// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint, type=warning, deprecated_member_use, deprecated_member_use_from_same_package
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'proton_download.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$ProtonDownloadProgress {

 Object get field0;



@override
bool operator ==(Object other) {
  final _this = this as ProtonDownloadProgress;
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ProtonDownloadProgress&&const DeepCollectionEquality().equals(other.field0, _this.field0));
}


@override
int get hashCode {
  final _this = this as ProtonDownloadProgress;
  return Object.hash(runtimeType,const DeepCollectionEquality().hash(_this.field0));
}

@override
String toString() {
  final _this = this as ProtonDownloadProgress;
  return 'ProtonDownloadProgress(field0: ${_this.field0})';
}


}

/// @nodoc
class $ProtonDownloadProgressCopyWith<$Res>  {
$ProtonDownloadProgressCopyWith(ProtonDownloadProgress _, $Res Function(ProtonDownloadProgress) __);
}


/// Adds pattern-matching-related methods to [ProtonDownloadProgress].
extension ProtonDownloadProgressPatterns on ProtonDownloadProgress {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ProtonDownloadProgress_Started value)?  started,TResult Function( ProtonDownloadProgress_Progress value)?  progress,TResult Function( ProtonDownloadProgress_Extracted value)?  extracted,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ProtonDownloadProgress_Started() when started != null:
return started(_that);case ProtonDownloadProgress_Progress() when progress != null:
return progress(_that);case ProtonDownloadProgress_Extracted() when extracted != null:
return extracted(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ProtonDownloadProgress_Started value)  started,required TResult Function( ProtonDownloadProgress_Progress value)  progress,required TResult Function( ProtonDownloadProgress_Extracted value)  extracted,}){
final _that = this;
switch (_that) {
case ProtonDownloadProgress_Started():
return started(_that);case ProtonDownloadProgress_Progress():
return progress(_that);case ProtonDownloadProgress_Extracted():
return extracted(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ProtonDownloadProgress_Started value)?  started,TResult? Function( ProtonDownloadProgress_Progress value)?  progress,TResult? Function( ProtonDownloadProgress_Extracted value)?  extracted,}){
final _that = this;
switch (_that) {
case ProtonDownloadProgress_Started() when started != null:
return started(_that);case ProtonDownloadProgress_Progress() when progress != null:
return progress(_that);case ProtonDownloadProgress_Extracted() when extracted != null:
return extracted(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt field0)?  started,TResult Function( BigInt field0)?  progress,TResult Function( String field0)?  extracted,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ProtonDownloadProgress_Started() when started != null:
return started(_that.field0);case ProtonDownloadProgress_Progress() when progress != null:
return progress(_that.field0);case ProtonDownloadProgress_Extracted() when extracted != null:
return extracted(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt field0)  started,required TResult Function( BigInt field0)  progress,required TResult Function( String field0)  extracted,}) {final _that = this;
switch (_that) {
case ProtonDownloadProgress_Started():
return started(_that.field0);case ProtonDownloadProgress_Progress():
return progress(_that.field0);case ProtonDownloadProgress_Extracted():
return extracted(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt field0)?  started,TResult? Function( BigInt field0)?  progress,TResult? Function( String field0)?  extracted,}) {final _that = this;
switch (_that) {
case ProtonDownloadProgress_Started() when started != null:
return started(_that.field0);case ProtonDownloadProgress_Progress() when progress != null:
return progress(_that.field0);case ProtonDownloadProgress_Extracted() when extracted != null:
return extracted(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class ProtonDownloadProgress_Started extends ProtonDownloadProgress {
  const ProtonDownloadProgress_Started(this.field0): super._();
  

@override final  BigInt field0;

/// Create a copy of ProtonDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ProtonDownloadProgress_StartedCopyWith<ProtonDownloadProgress_Started> get copyWith => _$ProtonDownloadProgress_StartedCopyWithImpl<ProtonDownloadProgress_Started>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is ProtonDownloadProgress_Started&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0);
}

@override
String toString() {
    return 'ProtonDownloadProgress.started(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ProtonDownloadProgress_StartedCopyWith<$Res> implements $ProtonDownloadProgressCopyWith<$Res> {
  factory $ProtonDownloadProgress_StartedCopyWith(ProtonDownloadProgress_Started value, $Res Function(ProtonDownloadProgress_Started) _then) = _$ProtonDownloadProgress_StartedCopyWithImpl;
@useResult
$Res call({
 BigInt field0
});




}
/// @nodoc
class _$ProtonDownloadProgress_StartedCopyWithImpl<$Res>
    implements $ProtonDownloadProgress_StartedCopyWith<$Res> {
  _$ProtonDownloadProgress_StartedCopyWithImpl(this._self, this._then);

  final ProtonDownloadProgress_Started _self;
  final $Res Function(ProtonDownloadProgress_Started) _then;

/// Create a copy of ProtonDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ProtonDownloadProgress_Started(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class ProtonDownloadProgress_Progress extends ProtonDownloadProgress {
  const ProtonDownloadProgress_Progress(this.field0): super._();
  

@override final  BigInt field0;

/// Create a copy of ProtonDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ProtonDownloadProgress_ProgressCopyWith<ProtonDownloadProgress_Progress> get copyWith => _$ProtonDownloadProgress_ProgressCopyWithImpl<ProtonDownloadProgress_Progress>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is ProtonDownloadProgress_Progress&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0);
}

@override
String toString() {
    return 'ProtonDownloadProgress.progress(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ProtonDownloadProgress_ProgressCopyWith<$Res> implements $ProtonDownloadProgressCopyWith<$Res> {
  factory $ProtonDownloadProgress_ProgressCopyWith(ProtonDownloadProgress_Progress value, $Res Function(ProtonDownloadProgress_Progress) _then) = _$ProtonDownloadProgress_ProgressCopyWithImpl;
@useResult
$Res call({
 BigInt field0
});




}
/// @nodoc
class _$ProtonDownloadProgress_ProgressCopyWithImpl<$Res>
    implements $ProtonDownloadProgress_ProgressCopyWith<$Res> {
  _$ProtonDownloadProgress_ProgressCopyWithImpl(this._self, this._then);

  final ProtonDownloadProgress_Progress _self;
  final $Res Function(ProtonDownloadProgress_Progress) _then;

/// Create a copy of ProtonDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ProtonDownloadProgress_Progress(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class ProtonDownloadProgress_Extracted extends ProtonDownloadProgress {
  const ProtonDownloadProgress_Extracted(this.field0): super._();
  

@override final  String field0;

/// Create a copy of ProtonDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ProtonDownloadProgress_ExtractedCopyWith<ProtonDownloadProgress_Extracted> get copyWith => _$ProtonDownloadProgress_ExtractedCopyWithImpl<ProtonDownloadProgress_Extracted>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is ProtonDownloadProgress_Extracted&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0);
}

@override
String toString() {
    return 'ProtonDownloadProgress.extracted(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ProtonDownloadProgress_ExtractedCopyWith<$Res> implements $ProtonDownloadProgressCopyWith<$Res> {
  factory $ProtonDownloadProgress_ExtractedCopyWith(ProtonDownloadProgress_Extracted value, $Res Function(ProtonDownloadProgress_Extracted) _then) = _$ProtonDownloadProgress_ExtractedCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ProtonDownloadProgress_ExtractedCopyWithImpl<$Res>
    implements $ProtonDownloadProgress_ExtractedCopyWith<$Res> {
  _$ProtonDownloadProgress_ExtractedCopyWithImpl(this._self, this._then);

  final ProtonDownloadProgress_Extracted _self;
  final $Res Function(ProtonDownloadProgress_Extracted) _then;

/// Create a copy of ProtonDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ProtonDownloadProgress_Extracted(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
