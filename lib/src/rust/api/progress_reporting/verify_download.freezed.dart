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





@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'VerifyDownloadProgress()';
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( VerifyDownloadProgress_Started value)?  started,TResult Function( VerifyDownloadProgress_Progress value)?  progress,TResult Function( VerifyDownloadProgress_CouldNotResolvePath value)?  couldNotResolvePath,TResult Function( VerifyDownloadProgress_FileNotFound value)?  fileNotFound,TResult Function( VerifyDownloadProgress_ChecksumMismatch value)?  checksumMismatch,TResult Function( VerifyDownloadProgress_Finished value)?  finished,TResult Function( VerifyDownloadProgress_Cancelled value)?  cancelled,required TResult orElse(),}){
final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started() when started != null:
return started(_that);case VerifyDownloadProgress_Progress() when progress != null:
return progress(_that);case VerifyDownloadProgress_CouldNotResolvePath() when couldNotResolvePath != null:
return couldNotResolvePath(_that);case VerifyDownloadProgress_FileNotFound() when fileNotFound != null:
return fileNotFound(_that);case VerifyDownloadProgress_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that);case VerifyDownloadProgress_Finished() when finished != null:
return finished(_that);case VerifyDownloadProgress_Cancelled() when cancelled != null:
return cancelled(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( VerifyDownloadProgress_Started value)  started,required TResult Function( VerifyDownloadProgress_Progress value)  progress,required TResult Function( VerifyDownloadProgress_CouldNotResolvePath value)  couldNotResolvePath,required TResult Function( VerifyDownloadProgress_FileNotFound value)  fileNotFound,required TResult Function( VerifyDownloadProgress_ChecksumMismatch value)  checksumMismatch,required TResult Function( VerifyDownloadProgress_Finished value)  finished,required TResult Function( VerifyDownloadProgress_Cancelled value)  cancelled,}){
final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started():
return started(_that);case VerifyDownloadProgress_Progress():
return progress(_that);case VerifyDownloadProgress_CouldNotResolvePath():
return couldNotResolvePath(_that);case VerifyDownloadProgress_FileNotFound():
return fileNotFound(_that);case VerifyDownloadProgress_ChecksumMismatch():
return checksumMismatch(_that);case VerifyDownloadProgress_Finished():
return finished(_that);case VerifyDownloadProgress_Cancelled():
return cancelled(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( VerifyDownloadProgress_Started value)?  started,TResult? Function( VerifyDownloadProgress_Progress value)?  progress,TResult? Function( VerifyDownloadProgress_CouldNotResolvePath value)?  couldNotResolvePath,TResult? Function( VerifyDownloadProgress_FileNotFound value)?  fileNotFound,TResult? Function( VerifyDownloadProgress_ChecksumMismatch value)?  checksumMismatch,TResult? Function( VerifyDownloadProgress_Finished value)?  finished,TResult? Function( VerifyDownloadProgress_Cancelled value)?  cancelled,}){
final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started() when started != null:
return started(_that);case VerifyDownloadProgress_Progress() when progress != null:
return progress(_that);case VerifyDownloadProgress_CouldNotResolvePath() when couldNotResolvePath != null:
return couldNotResolvePath(_that);case VerifyDownloadProgress_FileNotFound() when fileNotFound != null:
return fileNotFound(_that);case VerifyDownloadProgress_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that);case VerifyDownloadProgress_Finished() when finished != null:
return finished(_that);case VerifyDownloadProgress_Cancelled() when cancelled != null:
return cancelled(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt totalBytes)?  started,TResult Function( BigInt checkedBytes)?  progress,TResult Function( String path,  BigInt chunkSize)?  couldNotResolvePath,TResult Function( String path,  BigInt chunkSize)?  fileNotFound,TResult Function( String path,  BigInt chunkSize)?  checksumMismatch,TResult Function( BigInt badChunks)?  finished,TResult Function()?  cancelled,required TResult orElse(),}) {final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started() when started != null:
return started(_that.totalBytes);case VerifyDownloadProgress_Progress() when progress != null:
return progress(_that.checkedBytes);case VerifyDownloadProgress_CouldNotResolvePath() when couldNotResolvePath != null:
return couldNotResolvePath(_that.path,_that.chunkSize);case VerifyDownloadProgress_FileNotFound() when fileNotFound != null:
return fileNotFound(_that.path,_that.chunkSize);case VerifyDownloadProgress_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that.path,_that.chunkSize);case VerifyDownloadProgress_Finished() when finished != null:
return finished(_that.badChunks);case VerifyDownloadProgress_Cancelled() when cancelled != null:
return cancelled();case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt totalBytes)  started,required TResult Function( BigInt checkedBytes)  progress,required TResult Function( String path,  BigInt chunkSize)  couldNotResolvePath,required TResult Function( String path,  BigInt chunkSize)  fileNotFound,required TResult Function( String path,  BigInt chunkSize)  checksumMismatch,required TResult Function( BigInt badChunks)  finished,required TResult Function()  cancelled,}) {final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started():
return started(_that.totalBytes);case VerifyDownloadProgress_Progress():
return progress(_that.checkedBytes);case VerifyDownloadProgress_CouldNotResolvePath():
return couldNotResolvePath(_that.path,_that.chunkSize);case VerifyDownloadProgress_FileNotFound():
return fileNotFound(_that.path,_that.chunkSize);case VerifyDownloadProgress_ChecksumMismatch():
return checksumMismatch(_that.path,_that.chunkSize);case VerifyDownloadProgress_Finished():
return finished(_that.badChunks);case VerifyDownloadProgress_Cancelled():
return cancelled();}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt totalBytes)?  started,TResult? Function( BigInt checkedBytes)?  progress,TResult? Function( String path,  BigInt chunkSize)?  couldNotResolvePath,TResult? Function( String path,  BigInt chunkSize)?  fileNotFound,TResult? Function( String path,  BigInt chunkSize)?  checksumMismatch,TResult? Function( BigInt badChunks)?  finished,TResult? Function()?  cancelled,}) {final _that = this;
switch (_that) {
case VerifyDownloadProgress_Started() when started != null:
return started(_that.totalBytes);case VerifyDownloadProgress_Progress() when progress != null:
return progress(_that.checkedBytes);case VerifyDownloadProgress_CouldNotResolvePath() when couldNotResolvePath != null:
return couldNotResolvePath(_that.path,_that.chunkSize);case VerifyDownloadProgress_FileNotFound() when fileNotFound != null:
return fileNotFound(_that.path,_that.chunkSize);case VerifyDownloadProgress_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that.path,_that.chunkSize);case VerifyDownloadProgress_Finished() when finished != null:
return finished(_that.badChunks);case VerifyDownloadProgress_Cancelled() when cancelled != null:
return cancelled();case _:
  return null;

}
}

}

/// @nodoc


class VerifyDownloadProgress_Started extends VerifyDownloadProgress {
  const VerifyDownloadProgress_Started({required this.totalBytes}): super._();
  

 final  BigInt totalBytes;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_StartedCopyWith<VerifyDownloadProgress_Started> get copyWith => _$VerifyDownloadProgress_StartedCopyWithImpl<VerifyDownloadProgress_Started>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_Started&&(identical(other.totalBytes, totalBytes) || other.totalBytes == totalBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,totalBytes);
}

@override
String toString() {
    return 'VerifyDownloadProgress.started(totalBytes: $totalBytes)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_StartedCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_StartedCopyWith(VerifyDownloadProgress_Started value, $Res Function(VerifyDownloadProgress_Started) _then) = _$VerifyDownloadProgress_StartedCopyWithImpl;
@useResult
$Res call({
 BigInt totalBytes
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
@pragma('vm:prefer-inline') $Res call({Object? totalBytes = null,}) {
  return _then(VerifyDownloadProgress_Started(
totalBytes: null == totalBytes ? _self.totalBytes : totalBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_Progress extends VerifyDownloadProgress {
  const VerifyDownloadProgress_Progress({required this.checkedBytes}): super._();
  

 final  BigInt checkedBytes;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_ProgressCopyWith<VerifyDownloadProgress_Progress> get copyWith => _$VerifyDownloadProgress_ProgressCopyWithImpl<VerifyDownloadProgress_Progress>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_Progress&&(identical(other.checkedBytes, checkedBytes) || other.checkedBytes == checkedBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,checkedBytes);
}

@override
String toString() {
    return 'VerifyDownloadProgress.progress(checkedBytes: $checkedBytes)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_ProgressCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_ProgressCopyWith(VerifyDownloadProgress_Progress value, $Res Function(VerifyDownloadProgress_Progress) _then) = _$VerifyDownloadProgress_ProgressCopyWithImpl;
@useResult
$Res call({
 BigInt checkedBytes
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
@pragma('vm:prefer-inline') $Res call({Object? checkedBytes = null,}) {
  return _then(VerifyDownloadProgress_Progress(
checkedBytes: null == checkedBytes ? _self.checkedBytes : checkedBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_CouldNotResolvePath extends VerifyDownloadProgress {
  const VerifyDownloadProgress_CouldNotResolvePath({required this.path, required this.chunkSize}): super._();
  

 final  String path;
 final  BigInt chunkSize;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_CouldNotResolvePathCopyWith<VerifyDownloadProgress_CouldNotResolvePath> get copyWith => _$VerifyDownloadProgress_CouldNotResolvePathCopyWithImpl<VerifyDownloadProgress_CouldNotResolvePath>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_CouldNotResolvePath&&(identical(other.path, path) || other.path == path)&&(identical(other.chunkSize, chunkSize) || other.chunkSize == chunkSize));
}


@override
int get hashCode {
    return Object.hash(runtimeType,path,chunkSize);
}

@override
String toString() {
    return 'VerifyDownloadProgress.couldNotResolvePath(path: $path, chunkSize: $chunkSize)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_CouldNotResolvePathCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_CouldNotResolvePathCopyWith(VerifyDownloadProgress_CouldNotResolvePath value, $Res Function(VerifyDownloadProgress_CouldNotResolvePath) _then) = _$VerifyDownloadProgress_CouldNotResolvePathCopyWithImpl;
@useResult
$Res call({
 String path, BigInt chunkSize
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
@pragma('vm:prefer-inline') $Res call({Object? path = null,Object? chunkSize = null,}) {
  return _then(VerifyDownloadProgress_CouldNotResolvePath(
path: null == path ? _self.path : path // ignore: cast_nullable_to_non_nullable
as String,chunkSize: null == chunkSize ? _self.chunkSize : chunkSize // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_FileNotFound extends VerifyDownloadProgress {
  const VerifyDownloadProgress_FileNotFound({required this.path, required this.chunkSize}): super._();
  

 final  String path;
 final  BigInt chunkSize;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_FileNotFoundCopyWith<VerifyDownloadProgress_FileNotFound> get copyWith => _$VerifyDownloadProgress_FileNotFoundCopyWithImpl<VerifyDownloadProgress_FileNotFound>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_FileNotFound&&(identical(other.path, path) || other.path == path)&&(identical(other.chunkSize, chunkSize) || other.chunkSize == chunkSize));
}


@override
int get hashCode {
    return Object.hash(runtimeType,path,chunkSize);
}

@override
String toString() {
    return 'VerifyDownloadProgress.fileNotFound(path: $path, chunkSize: $chunkSize)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_FileNotFoundCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_FileNotFoundCopyWith(VerifyDownloadProgress_FileNotFound value, $Res Function(VerifyDownloadProgress_FileNotFound) _then) = _$VerifyDownloadProgress_FileNotFoundCopyWithImpl;
@useResult
$Res call({
 String path, BigInt chunkSize
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
@pragma('vm:prefer-inline') $Res call({Object? path = null,Object? chunkSize = null,}) {
  return _then(VerifyDownloadProgress_FileNotFound(
path: null == path ? _self.path : path // ignore: cast_nullable_to_non_nullable
as String,chunkSize: null == chunkSize ? _self.chunkSize : chunkSize // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_ChecksumMismatch extends VerifyDownloadProgress {
  const VerifyDownloadProgress_ChecksumMismatch({required this.path, required this.chunkSize}): super._();
  

 final  String path;
 final  BigInt chunkSize;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_ChecksumMismatchCopyWith<VerifyDownloadProgress_ChecksumMismatch> get copyWith => _$VerifyDownloadProgress_ChecksumMismatchCopyWithImpl<VerifyDownloadProgress_ChecksumMismatch>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_ChecksumMismatch&&(identical(other.path, path) || other.path == path)&&(identical(other.chunkSize, chunkSize) || other.chunkSize == chunkSize));
}


@override
int get hashCode {
    return Object.hash(runtimeType,path,chunkSize);
}

@override
String toString() {
    return 'VerifyDownloadProgress.checksumMismatch(path: $path, chunkSize: $chunkSize)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_ChecksumMismatchCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_ChecksumMismatchCopyWith(VerifyDownloadProgress_ChecksumMismatch value, $Res Function(VerifyDownloadProgress_ChecksumMismatch) _then) = _$VerifyDownloadProgress_ChecksumMismatchCopyWithImpl;
@useResult
$Res call({
 String path, BigInt chunkSize
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
@pragma('vm:prefer-inline') $Res call({Object? path = null,Object? chunkSize = null,}) {
  return _then(VerifyDownloadProgress_ChecksumMismatch(
path: null == path ? _self.path : path // ignore: cast_nullable_to_non_nullable
as String,chunkSize: null == chunkSize ? _self.chunkSize : chunkSize // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_Finished extends VerifyDownloadProgress {
  const VerifyDownloadProgress_Finished({required this.badChunks}): super._();
  

 final  BigInt badChunks;

/// Create a copy of VerifyDownloadProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$VerifyDownloadProgress_FinishedCopyWith<VerifyDownloadProgress_Finished> get copyWith => _$VerifyDownloadProgress_FinishedCopyWithImpl<VerifyDownloadProgress_Finished>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_Finished&&(identical(other.badChunks, badChunks) || other.badChunks == badChunks));
}


@override
int get hashCode {
    return Object.hash(runtimeType,badChunks);
}

@override
String toString() {
    return 'VerifyDownloadProgress.finished(badChunks: $badChunks)';
}


}

/// @nodoc
abstract mixin class $VerifyDownloadProgress_FinishedCopyWith<$Res> implements $VerifyDownloadProgressCopyWith<$Res> {
  factory $VerifyDownloadProgress_FinishedCopyWith(VerifyDownloadProgress_Finished value, $Res Function(VerifyDownloadProgress_Finished) _then) = _$VerifyDownloadProgress_FinishedCopyWithImpl;
@useResult
$Res call({
 BigInt badChunks
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
@pragma('vm:prefer-inline') $Res call({Object? badChunks = null,}) {
  return _then(VerifyDownloadProgress_Finished(
badChunks: null == badChunks ? _self.badChunks : badChunks // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class VerifyDownloadProgress_Cancelled extends VerifyDownloadProgress {
  const VerifyDownloadProgress_Cancelled(): super._();
  






@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is VerifyDownloadProgress_Cancelled);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'VerifyDownloadProgress.cancelled()';
}


}




// dart format on
