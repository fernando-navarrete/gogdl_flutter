// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint, type=warning, deprecated_member_use, deprecated_member_use_from_same_package
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'gogdl.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$GogError {

 String get message;
/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GogErrorCopyWith<GogError> get copyWith => _$GogErrorCopyWithImpl<GogError>(this as GogError, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GogError&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'GogError(message: $message)';
}


}

/// @nodoc
abstract mixin class $GogErrorCopyWith<$Res>  {
  factory $GogErrorCopyWith(GogError value, $Res Function(GogError) _then) = _$GogErrorCopyWithImpl;
@useResult
$Res call({
 String message
});




}
/// @nodoc
class _$GogErrorCopyWithImpl<$Res>
    implements $GogErrorCopyWith<$Res> {
  _$GogErrorCopyWithImpl(this._self, this._then);

  final GogError _self;
  final $Res Function(GogError) _then;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? message = null,}) {
  return _then(_self.copyWith(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [GogError].
extension GogErrorPatterns on GogError {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( GogError_AuthError value)?  authError,TResult Function( GogError_GameError value)?  gameError,TResult Function( GogError_DepotError value)?  depotError,TResult Function( GogError_SecureLinksError value)?  secureLinksError,TResult Function( GogError_DownloadError value)?  downloadError,TResult Function( GogError_SavesError value)?  savesError,required TResult orElse(),}){
final _that = this;
switch (_that) {
case GogError_AuthError() when authError != null:
return authError(_that);case GogError_GameError() when gameError != null:
return gameError(_that);case GogError_DepotError() when depotError != null:
return depotError(_that);case GogError_SecureLinksError() when secureLinksError != null:
return secureLinksError(_that);case GogError_DownloadError() when downloadError != null:
return downloadError(_that);case GogError_SavesError() when savesError != null:
return savesError(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( GogError_AuthError value)  authError,required TResult Function( GogError_GameError value)  gameError,required TResult Function( GogError_DepotError value)  depotError,required TResult Function( GogError_SecureLinksError value)  secureLinksError,required TResult Function( GogError_DownloadError value)  downloadError,required TResult Function( GogError_SavesError value)  savesError,}){
final _that = this;
switch (_that) {
case GogError_AuthError():
return authError(_that);case GogError_GameError():
return gameError(_that);case GogError_DepotError():
return depotError(_that);case GogError_SecureLinksError():
return secureLinksError(_that);case GogError_DownloadError():
return downloadError(_that);case GogError_SavesError():
return savesError(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( GogError_AuthError value)?  authError,TResult? Function( GogError_GameError value)?  gameError,TResult? Function( GogError_DepotError value)?  depotError,TResult? Function( GogError_SecureLinksError value)?  secureLinksError,TResult? Function( GogError_DownloadError value)?  downloadError,TResult? Function( GogError_SavesError value)?  savesError,}){
final _that = this;
switch (_that) {
case GogError_AuthError() when authError != null:
return authError(_that);case GogError_GameError() when gameError != null:
return gameError(_that);case GogError_DepotError() when depotError != null:
return depotError(_that);case GogError_SecureLinksError() when secureLinksError != null:
return secureLinksError(_that);case GogError_DownloadError() when downloadError != null:
return downloadError(_that);case GogError_SavesError() when savesError != null:
return savesError(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String message)?  authError,TResult Function( String message)?  gameError,TResult Function( String message)?  depotError,TResult Function( String message)?  secureLinksError,TResult Function( String message)?  downloadError,TResult Function( String message)?  savesError,required TResult orElse(),}) {final _that = this;
switch (_that) {
case GogError_AuthError() when authError != null:
return authError(_that.message);case GogError_GameError() when gameError != null:
return gameError(_that.message);case GogError_DepotError() when depotError != null:
return depotError(_that.message);case GogError_SecureLinksError() when secureLinksError != null:
return secureLinksError(_that.message);case GogError_DownloadError() when downloadError != null:
return downloadError(_that.message);case GogError_SavesError() when savesError != null:
return savesError(_that.message);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String message)  authError,required TResult Function( String message)  gameError,required TResult Function( String message)  depotError,required TResult Function( String message)  secureLinksError,required TResult Function( String message)  downloadError,required TResult Function( String message)  savesError,}) {final _that = this;
switch (_that) {
case GogError_AuthError():
return authError(_that.message);case GogError_GameError():
return gameError(_that.message);case GogError_DepotError():
return depotError(_that.message);case GogError_SecureLinksError():
return secureLinksError(_that.message);case GogError_DownloadError():
return downloadError(_that.message);case GogError_SavesError():
return savesError(_that.message);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String message)?  authError,TResult? Function( String message)?  gameError,TResult? Function( String message)?  depotError,TResult? Function( String message)?  secureLinksError,TResult? Function( String message)?  downloadError,TResult? Function( String message)?  savesError,}) {final _that = this;
switch (_that) {
case GogError_AuthError() when authError != null:
return authError(_that.message);case GogError_GameError() when gameError != null:
return gameError(_that.message);case GogError_DepotError() when depotError != null:
return depotError(_that.message);case GogError_SecureLinksError() when secureLinksError != null:
return secureLinksError(_that.message);case GogError_DownloadError() when downloadError != null:
return downloadError(_that.message);case GogError_SavesError() when savesError != null:
return savesError(_that.message);case _:
  return null;

}
}

}

/// @nodoc


class GogError_AuthError extends GogError {
  const GogError_AuthError({required this.message}): super._();
  

@override final  String message;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GogError_AuthErrorCopyWith<GogError_AuthError> get copyWith => _$GogError_AuthErrorCopyWithImpl<GogError_AuthError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GogError_AuthError&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'GogError.authError(message: $message)';
}


}

/// @nodoc
abstract mixin class $GogError_AuthErrorCopyWith<$Res> implements $GogErrorCopyWith<$Res> {
  factory $GogError_AuthErrorCopyWith(GogError_AuthError value, $Res Function(GogError_AuthError) _then) = _$GogError_AuthErrorCopyWithImpl;
@override @useResult
$Res call({
 String message
});




}
/// @nodoc
class _$GogError_AuthErrorCopyWithImpl<$Res>
    implements $GogError_AuthErrorCopyWith<$Res> {
  _$GogError_AuthErrorCopyWithImpl(this._self, this._then);

  final GogError_AuthError _self;
  final $Res Function(GogError_AuthError) _then;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(GogError_AuthError(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GogError_GameError extends GogError {
  const GogError_GameError({required this.message}): super._();
  

@override final  String message;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GogError_GameErrorCopyWith<GogError_GameError> get copyWith => _$GogError_GameErrorCopyWithImpl<GogError_GameError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GogError_GameError&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'GogError.gameError(message: $message)';
}


}

/// @nodoc
abstract mixin class $GogError_GameErrorCopyWith<$Res> implements $GogErrorCopyWith<$Res> {
  factory $GogError_GameErrorCopyWith(GogError_GameError value, $Res Function(GogError_GameError) _then) = _$GogError_GameErrorCopyWithImpl;
@override @useResult
$Res call({
 String message
});




}
/// @nodoc
class _$GogError_GameErrorCopyWithImpl<$Res>
    implements $GogError_GameErrorCopyWith<$Res> {
  _$GogError_GameErrorCopyWithImpl(this._self, this._then);

  final GogError_GameError _self;
  final $Res Function(GogError_GameError) _then;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(GogError_GameError(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GogError_DepotError extends GogError {
  const GogError_DepotError({required this.message}): super._();
  

@override final  String message;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GogError_DepotErrorCopyWith<GogError_DepotError> get copyWith => _$GogError_DepotErrorCopyWithImpl<GogError_DepotError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GogError_DepotError&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'GogError.depotError(message: $message)';
}


}

/// @nodoc
abstract mixin class $GogError_DepotErrorCopyWith<$Res> implements $GogErrorCopyWith<$Res> {
  factory $GogError_DepotErrorCopyWith(GogError_DepotError value, $Res Function(GogError_DepotError) _then) = _$GogError_DepotErrorCopyWithImpl;
@override @useResult
$Res call({
 String message
});




}
/// @nodoc
class _$GogError_DepotErrorCopyWithImpl<$Res>
    implements $GogError_DepotErrorCopyWith<$Res> {
  _$GogError_DepotErrorCopyWithImpl(this._self, this._then);

  final GogError_DepotError _self;
  final $Res Function(GogError_DepotError) _then;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(GogError_DepotError(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GogError_SecureLinksError extends GogError {
  const GogError_SecureLinksError({required this.message}): super._();
  

@override final  String message;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GogError_SecureLinksErrorCopyWith<GogError_SecureLinksError> get copyWith => _$GogError_SecureLinksErrorCopyWithImpl<GogError_SecureLinksError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GogError_SecureLinksError&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'GogError.secureLinksError(message: $message)';
}


}

/// @nodoc
abstract mixin class $GogError_SecureLinksErrorCopyWith<$Res> implements $GogErrorCopyWith<$Res> {
  factory $GogError_SecureLinksErrorCopyWith(GogError_SecureLinksError value, $Res Function(GogError_SecureLinksError) _then) = _$GogError_SecureLinksErrorCopyWithImpl;
@override @useResult
$Res call({
 String message
});




}
/// @nodoc
class _$GogError_SecureLinksErrorCopyWithImpl<$Res>
    implements $GogError_SecureLinksErrorCopyWith<$Res> {
  _$GogError_SecureLinksErrorCopyWithImpl(this._self, this._then);

  final GogError_SecureLinksError _self;
  final $Res Function(GogError_SecureLinksError) _then;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(GogError_SecureLinksError(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GogError_DownloadError extends GogError {
  const GogError_DownloadError({required this.message}): super._();
  

@override final  String message;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GogError_DownloadErrorCopyWith<GogError_DownloadError> get copyWith => _$GogError_DownloadErrorCopyWithImpl<GogError_DownloadError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GogError_DownloadError&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'GogError.downloadError(message: $message)';
}


}

/// @nodoc
abstract mixin class $GogError_DownloadErrorCopyWith<$Res> implements $GogErrorCopyWith<$Res> {
  factory $GogError_DownloadErrorCopyWith(GogError_DownloadError value, $Res Function(GogError_DownloadError) _then) = _$GogError_DownloadErrorCopyWithImpl;
@override @useResult
$Res call({
 String message
});




}
/// @nodoc
class _$GogError_DownloadErrorCopyWithImpl<$Res>
    implements $GogError_DownloadErrorCopyWith<$Res> {
  _$GogError_DownloadErrorCopyWithImpl(this._self, this._then);

  final GogError_DownloadError _self;
  final $Res Function(GogError_DownloadError) _then;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(GogError_DownloadError(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class GogError_SavesError extends GogError {
  const GogError_SavesError({required this.message}): super._();


@override final  String message;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$GogError_SavesErrorCopyWith<GogError_SavesError> get copyWith => _$GogError_SavesErrorCopyWithImpl<GogError_SavesError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is GogError_SavesError&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'GogError.savesError(message: $message)';
}


}

/// @nodoc
abstract mixin class $GogError_SavesErrorCopyWith<$Res> implements $GogErrorCopyWith<$Res> {
  factory $GogError_SavesErrorCopyWith(GogError_SavesError value, $Res Function(GogError_SavesError) _then) = _$GogError_SavesErrorCopyWithImpl;
@override @useResult
$Res call({
 String message
});




}
/// @nodoc
class _$GogError_SavesErrorCopyWithImpl<$Res>
    implements $GogError_SavesErrorCopyWith<$Res> {
  _$GogError_SavesErrorCopyWithImpl(this._self, this._then);

  final GogError_SavesError _self;
  final $Res Function(GogError_SavesError) _then;

/// Create a copy of GogError
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(GogError_SavesError(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
