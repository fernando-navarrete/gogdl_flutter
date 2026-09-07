// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint, type=warning, deprecated_member_use, deprecated_member_use_from_same_package
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'verify_download.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$VerifyDownloadProgress {

 Object get field0;



@override
bool operator ==(Object other) {
  final _this = this as VerifyDownloadProgress;
  return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress&&const DeepCollectionEquality().equals(other.field0, _this.field0));
}


@override
int get hashCode {
  final _this = this as VerifyDownloadProgress;
  return Object.hash(runtimeType,const DeepCollectionEquality().hash(_this.field0));
}

@override
String toString() {
  final _this = this as VerifyDownloadProgress;
  return 'VerifyDownloadProgress(field0: ${_this.field0})';
}


}

/// @nodoc
class $VerifyDownloadProgressCopyWith<$Res>  {
$VerifyDownloadProgressCopyWith(VerifyDownloadProgress _, $Res Function(VerifyDownloadProgress) __);
}


/// Adds pattern-matching-related methods to [VerifyDownloadProgress].
extension VerifyDownloadProgressPatterns on VerifyDownloadProgress {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( VerifyDownloadProgress_Started value)?  started,TResult Function( VerifyDownloadProgress_Progress value)?  progress,TResult Function( VerifyDownloadProgress_CouldNotResolvePath value)?  couldNotResolvePath,TResult Function( VerifyDownloadProgress_FileNotFound value)?  fileNotFound,TResult Function( VerifyDownloadProgress_ChecksumMismatch value)?  checksumMismatch,TResult Function( VerifyDownloadProgress_Finished value)?  finished,required TResult orElse(),}){
final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started() when started != null:
return started(_that);case VerifyDownloadProgress_Progress() when progress != null:
return progress(_that);case VerifyDownloadProgress_CouldNotResolvePath() when couldNotResolvePath != null:
return couldNotResolvePath(_that);case VerifyDownloadProgress_FileNotFound() when fileNotFound != null:
return fileNotFound(_that);case VerifyDownloadProgress_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that);case VerifyDownloadProgress_Finished() when finished != null:
return finished(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( VerifyDownloadProgress_Started value)  started,required TResult Function( VerifyDownloadProgress_Progress value)  progress,required TResult Function( VerifyDownloadProgress_CouldNotResolvePath value)  couldNotResolvePath,required TResult Function( VerifyDownloadProgress_FileNotFound value)  fileNotFound,required TResult Function( VerifyDownloadProgress_ChecksumMismatch value)  checksumMismatch,required TResult Function( VerifyDownloadProgress_Finished value)  finished,}){
final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started():
return started(_that);case VerifyDownloadProgress_Progress():
return progress(_that);case VerifyDownloadProgress_CouldNotResolvePath():
return couldNotResolvePath(_that);case VerifyDownloadProgress_FileNotFound():
return fileNotFound(_that);case VerifyDownloadProgress_ChecksumMismatch():
return checksumMismatch(_that);case VerifyDownloadProgress_Finished():
return finished(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( VerifyDownloadProgress_Started value)?  started,TResult? Function( VerifyDownloadProgress_Progress value)?  progress,TResult? Function( VerifyDownloadProgress_CouldNotResolvePath value)?  couldNotResolvePath,TResult? Function( VerifyDownloadProgress_FileNotFound value)?  fileNotFound,TResult? Function( VerifyDownloadProgress_ChecksumMismatch value)?  checksumMismatch,TResult? Function( VerifyDownloadProgress_Finished value)?  finished,}){
final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started() when started != null:
return started(_that);case VerifyDownloadProgress_Progress() when progress != null:
return progress(_that);case VerifyDownloadProgress_CouldNotResolvePath() when couldNotResolvePath != null:
return couldNotResolvePath(_that);case VerifyDownloadProgress_FileNotFound() when fileNotFound != null:
return fileNotFound(_that);case VerifyDownloadProgress_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that);case VerifyDownloadProgress_Finished() when finished != null:
return finished(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt field0)?  started,TResult Function( BigInt field0)?  progress,TResult Function( String field0,  BigInt field1)?  couldNotResolvePath,TResult Function( String field0,  BigInt field1)?  fileNotFound,TResult Function( String field0,  BigInt field1)?  checksumMismatch,TResult Function( BigInt field0)?  finished,required TResult orElse(),}) {final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started() when started != null:
return started(_that.field0);case VerifyDownloadProgress_Progress() when progress != null:
return progress(_that.field0);case VerifyDownloadProgress_CouldNotResolvePath() when couldNotResolvePath != null:
return couldNotResolvePath(_that.field0,_that.field1);case VerifyDownloadProgress_FileNotFound() when fileNotFound != null:
return fileNotFound(_that.field0,_that.field1);case VerifyDownloadProgress_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that.field0,_that.field1);case VerifyDownloadProgress_Finished() when finished != null:
return finished(_that.field0);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt field0)  started,required TResult Function( BigInt field0)  progress,required TResult Function( String field0,  BigInt field1)  couldNotResolvePath,required TResult Function( String field0,  BigInt field1)  fileNotFound,required TResult Function( String field0,  BigInt field1)  checksumMismatch,required TResult Function( BigInt field0)  finished,}) {final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started():
return started(_that.field0);case VerifyDownloadProgress_Progress():
return progress(_that.field0);case VerifyDownloadProgress_CouldNotResolvePath():
return couldNotResolvePath(_that.field0,_that.field1);case VerifyDownloadProgress_FileNotFound():
return fileNotFound(_that.field0,_that.field1);case VerifyDownloadProgress_ChecksumMismatch():
return checksumMismatch(_that.field0,_that.field1);case VerifyDownloadProgress_Finished():
return finished(_that.field0);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt field0)?  started,TResult? Function( BigInt field0)?  progress,TResult? Function( String field0,  BigInt field1)?  couldNotResolvePath,TResult? Function( String field0,  BigInt field1)?  fileNotFound,TResult? Function( String field0,  BigInt field1)?  checksumMismatch,TResult? Function( BigInt field0)?  finished,}) {final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started() when started != null:
return started(_that.field0);case VerifyDownloadProgress_Progress() when progress != null:
return progress(_that.field0);case VerifyDownloadProgress_CouldNotResolvePath() when couldNotResolvePath != null:
return couldNotResolvePath(_that.field0,_that.field1);case VerifyDownloadProgress_FileNotFound() when fileNotFound != null:
return fileNotFound(_that.field0,_that.field1);case VerifyDownloadProgress_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that.field0,_that.field1);case VerifyDownloadProgress_Finished() when finished != null:
return finished(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class VerifyDownloadProgress_Started extends VerifyDownloadProgress {
  const VerifyDownloadProgress_Started(this.field0): super._();
  

@override final  BigInt field0;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_StartedCopyWith<VerifyDownloadProgress_Started> get copyWith => _$VerifyDownloadProgress_StartedCopyWithImpl<VerifyDownloadProgress_Started>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_Started&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0);
}

@override
String toString() {
    return 'VerifyDownloadProgress.started(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_StartedCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_StartedCopyWith(VerifyDownloadProgress_Started value, $Res Function(VerifyDownloadProgress_Started) _then) = _$VerifyDownloadProgress_StartedCopyWithImpl;
@useResult
$Res call({
 BigInt field0
});




}
/// @nodoc
class _$VerifyDownloadProgress_StartedCopyWithImpl<$Res>
    implements $VerifyDownloadProgress_StartedCopyWith<$Res> {
  _$VerifyDownloadProgress_StartedCopyWithImpl(this._self, this._then);

  final VerifyDownloadProgress_Started _self;
  final $Res Function(VerifyDownloadProgress_Started) _then;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(VerifyDownloadProgress_Started(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_Progress extends VerifyDownloadProgress {
  const VerifyDownloadProgress_Progress(this.field0): super._();
  

@override final  BigInt field0;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_ProgressCopyWith<VerifyDownloadProgress_Progress> get copyWith => _$VerifyDownloadProgress_ProgressCopyWithImpl<VerifyDownloadProgress_Progress>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_Progress&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0);
}

@override
String toString() {
    return 'VerifyDownloadProgress.progress(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_ProgressCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_ProgressCopyWith(VerifyDownloadProgress_Progress value, $Res Function(VerifyDownloadProgress_Progress) _then) = _$VerifyDownloadProgress_ProgressCopyWithImpl;
@useResult
$Res call({
 BigInt field0
});




}
/// @nodoc
class _$VerifyDownloadProgress_ProgressCopyWithImpl<$Res>
    implements $VerifyDownloadProgress_ProgressCopyWith<$Res> {
  _$VerifyDownloadProgress_ProgressCopyWithImpl(this._self, this._then);

  final VerifyDownloadProgress_Progress _self;
  final $Res Function(VerifyDownloadProgress_Progress) _then;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(VerifyDownloadProgress_Progress(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_CouldNotResolvePath extends VerifyDownloadProgress {
  const VerifyDownloadProgress_CouldNotResolvePath(this.field0, this.field1): super._();
  

@override final  String field0;
 final  BigInt field1;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_CouldNotResolvePathCopyWith<VerifyDownloadProgress_CouldNotResolvePath> get copyWith => _$VerifyDownloadProgress_CouldNotResolvePathCopyWithImpl<VerifyDownloadProgress_CouldNotResolvePath>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_CouldNotResolvePath&&(identical(other.field0, field0) || other.field0 == field0)&&(identical(other.field1, field1) || other.field1 == field1));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0,field1);
}

@override
String toString() {
    return 'VerifyDownloadProgress.couldNotResolvePath(field0: $field0, field1: $field1)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_CouldNotResolvePathCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_CouldNotResolvePathCopyWith(VerifyDownloadProgress_CouldNotResolvePath value, $Res Function(VerifyDownloadProgress_CouldNotResolvePath) _then) = _$VerifyDownloadProgress_CouldNotResolvePathCopyWithImpl;
@useResult
$Res call({
 String field0, BigInt field1
});




}
/// @nodoc
class _$VerifyDownloadProgress_CouldNotResolvePathCopyWithImpl<$Res>
    implements $VerifyDownloadProgress_CouldNotResolvePathCopyWith<$Res> {
  _$VerifyDownloadProgress_CouldNotResolvePathCopyWithImpl(this._self, this._then);

  final VerifyDownloadProgress_CouldNotResolvePath _self;
  final $Res Function(VerifyDownloadProgress_CouldNotResolvePath) _then;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,Object? field1 = null,}) {
  return _then(VerifyDownloadProgress_CouldNotResolvePath(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,null == field1 ? _self.field1 : field1 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_FileNotFound extends VerifyDownloadProgress {
  const VerifyDownloadProgress_FileNotFound(this.field0, this.field1): super._();
  

@override final  String field0;
 final  BigInt field1;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_FileNotFoundCopyWith<VerifyDownloadProgress_FileNotFound> get copyWith => _$VerifyDownloadProgress_FileNotFoundCopyWithImpl<VerifyDownloadProgress_FileNotFound>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_FileNotFound&&(identical(other.field0, field0) || other.field0 == field0)&&(identical(other.field1, field1) || other.field1 == field1));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0,field1);
}

@override
String toString() {
    return 'VerifyDownloadProgress.fileNotFound(field0: $field0, field1: $field1)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_FileNotFoundCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_FileNotFoundCopyWith(VerifyDownloadProgress_FileNotFound value, $Res Function(VerifyDownloadProgress_FileNotFound) _then) = _$VerifyDownloadProgress_FileNotFoundCopyWithImpl;
@useResult
$Res call({
 String field0, BigInt field1
});




}
/// @nodoc
class _$VerifyDownloadProgress_FileNotFoundCopyWithImpl<$Res>
    implements $VerifyDownloadProgress_FileNotFoundCopyWith<$Res> {
  _$VerifyDownloadProgress_FileNotFoundCopyWithImpl(this._self, this._then);

  final VerifyDownloadProgress_FileNotFound _self;
  final $Res Function(VerifyDownloadProgress_FileNotFound) _then;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,Object? field1 = null,}) {
  return _then(VerifyDownloadProgress_FileNotFound(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,null == field1 ? _self.field1 : field1 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_ChecksumMismatch extends VerifyDownloadProgress {
  const VerifyDownloadProgress_ChecksumMismatch(this.field0, this.field1): super._();
  

@override final  String field0;
 final  BigInt field1;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_ChecksumMismatchCopyWith<VerifyDownloadProgress_ChecksumMismatch> get copyWith => _$VerifyDownloadProgress_ChecksumMismatchCopyWithImpl<VerifyDownloadProgress_ChecksumMismatch>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_ChecksumMismatch&&(identical(other.field0, field0) || other.field0 == field0)&&(identical(other.field1, field1) || other.field1 == field1));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0,field1);
}

@override
String toString() {
    return 'VerifyDownloadProgress.checksumMismatch(field0: $field0, field1: $field1)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_ChecksumMismatchCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_ChecksumMismatchCopyWith(VerifyDownloadProgress_ChecksumMismatch value, $Res Function(VerifyDownloadProgress_ChecksumMismatch) _then) = _$VerifyDownloadProgress_ChecksumMismatchCopyWithImpl;
@useResult
$Res call({
 String field0, BigInt field1
});




}
/// @nodoc
class _$VerifyDownloadProgress_ChecksumMismatchCopyWithImpl<$Res>
    implements $VerifyDownloadProgress_ChecksumMismatchCopyWith<$Res> {
  _$VerifyDownloadProgress_ChecksumMismatchCopyWithImpl(this._self, this._then);

  final VerifyDownloadProgress_ChecksumMismatch _self;
  final $Res Function(VerifyDownloadProgress_ChecksumMismatch) _then;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,Object? field1 = null,}) {
  return _then(VerifyDownloadProgress_ChecksumMismatch(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,null == field1 ? _self.field1 : field1 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_Finished extends VerifyDownloadProgress {
  const VerifyDownloadProgress_Finished(this.field0): super._();
  

@override final  BigInt field0;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_FinishedCopyWith<VerifyDownloadProgress_Finished> get copyWith => _$VerifyDownloadProgress_FinishedCopyWithImpl<VerifyDownloadProgress_Finished>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_Finished&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode {
    return Object.hash(runtimeType,field0);
}

@override
String toString() {
    return 'VerifyDownloadProgress.finished(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_FinishedCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_FinishedCopyWith(VerifyDownloadProgress_Finished value, $Res Function(VerifyDownloadProgress_Finished) _then) = _$VerifyDownloadProgress_FinishedCopyWithImpl;
@useResult
$Res call({
 BigInt field0
});




}
/// @nodoc
class _$VerifyDownloadProgress_FinishedCopyWithImpl<$Res>
    implements $VerifyDownloadProgress_FinishedCopyWith<$Res> {
  _$VerifyDownloadProgress_FinishedCopyWithImpl(this._self, this._then);

  final VerifyDownloadProgress_Finished _self;
  final $Res Function(VerifyDownloadProgress_Finished) _then;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(VerifyDownloadProgress_Finished(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

// dart format on
