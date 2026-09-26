// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint, type=warning, deprecated_member_use, deprecated_member_use_from_same_package
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'saves_download.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$DownloadSavesProgress {





@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadSavesProgress);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'DownloadSavesProgress()';
}


}

/// @nodoc
class $DownloadSavesProgressCopyWith<$Res>  {
$DownloadSavesProgressCopyWith(DownloadSavesProgress _, $Res Function(DownloadSavesProgress) __);
}


/// Adds pattern-matching-related methods to [DownloadSavesProgress].
extension DownloadSavesProgressPatterns on DownloadSavesProgress {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( DownloadSavesProgress_Started value)?  started,TResult Function( DownloadSavesProgress_FileStarted value)?  fileStarted,TResult Function( DownloadSavesProgress_Progress value)?  progress,TResult Function( DownloadSavesProgress_FileFinished value)?  fileFinished,TResult Function( DownloadSavesProgress_Finished value)?  finished,TResult Function( DownloadSavesProgress_Cancelled value)?  cancelled,required TResult orElse(),}){
final _that = this;
switch (_that) {
case DownloadSavesProgress_Started() when started != null:
return started(_that);case DownloadSavesProgress_FileStarted() when fileStarted != null:
return fileStarted(_that);case DownloadSavesProgress_Progress() when progress != null:
return progress(_that);case DownloadSavesProgress_FileFinished() when fileFinished != null:
return fileFinished(_that);case DownloadSavesProgress_Finished() when finished != null:
return finished(_that);case DownloadSavesProgress_Cancelled() when cancelled != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( DownloadSavesProgress_Started value)  started,required TResult Function( DownloadSavesProgress_FileStarted value)  fileStarted,required TResult Function( DownloadSavesProgress_Progress value)  progress,required TResult Function( DownloadSavesProgress_FileFinished value)  fileFinished,required TResult Function( DownloadSavesProgress_Finished value)  finished,required TResult Function( DownloadSavesProgress_Cancelled value)  cancelled,}){
final _that = this;
switch (_that) {
case DownloadSavesProgress_Started():
return started(_that);case DownloadSavesProgress_FileStarted():
return fileStarted(_that);case DownloadSavesProgress_Progress():
return progress(_that);case DownloadSavesProgress_FileFinished():
return fileFinished(_that);case DownloadSavesProgress_Finished():
return finished(_that);case DownloadSavesProgress_Cancelled():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( DownloadSavesProgress_Started value)?  started,TResult? Function( DownloadSavesProgress_FileStarted value)?  fileStarted,TResult? Function( DownloadSavesProgress_Progress value)?  progress,TResult? Function( DownloadSavesProgress_FileFinished value)?  fileFinished,TResult? Function( DownloadSavesProgress_Finished value)?  finished,TResult? Function( DownloadSavesProgress_Cancelled value)?  cancelled,}){
final _that = this;
switch (_that) {
case DownloadSavesProgress_Started() when started != null:
return started(_that);case DownloadSavesProgress_FileStarted() when fileStarted != null:
return fileStarted(_that);case DownloadSavesProgress_Progress() when progress != null:
return progress(_that);case DownloadSavesProgress_FileFinished() when fileFinished != null:
return fileFinished(_that);case DownloadSavesProgress_Finished() when finished != null:
return finished(_that);case DownloadSavesProgress_Cancelled() when cancelled != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt totalFiles,  BigInt totalBytes)?  started,TResult Function( String name,  String destination,  BigInt totalBytes)?  fileStarted,TResult Function( BigInt downloadedBytes,  BigInt fileDownloadedBytes)?  progress,TResult Function( String name)?  fileFinished,TResult Function()?  finished,TResult Function()?  cancelled,required TResult orElse(),}) {final _that = this;
switch (_that) {
case DownloadSavesProgress_Started() when started != null:
return started(_that.totalFiles,_that.totalBytes);case DownloadSavesProgress_FileStarted() when fileStarted != null:
return fileStarted(_that.name,_that.destination,_that.totalBytes);case DownloadSavesProgress_Progress() when progress != null:
return progress(_that.downloadedBytes,_that.fileDownloadedBytes);case DownloadSavesProgress_FileFinished() when fileFinished != null:
return fileFinished(_that.name);case DownloadSavesProgress_Finished() when finished != null:
return finished();case DownloadSavesProgress_Cancelled() when cancelled != null:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt totalFiles,  BigInt totalBytes)  started,required TResult Function( String name,  String destination,  BigInt totalBytes)  fileStarted,required TResult Function( BigInt downloadedBytes,  BigInt fileDownloadedBytes)  progress,required TResult Function( String name)  fileFinished,required TResult Function()  finished,required TResult Function()  cancelled,}) {final _that = this;
switch (_that) {
case DownloadSavesProgress_Started():
return started(_that.totalFiles,_that.totalBytes);case DownloadSavesProgress_FileStarted():
return fileStarted(_that.name,_that.destination,_that.totalBytes);case DownloadSavesProgress_Progress():
return progress(_that.downloadedBytes,_that.fileDownloadedBytes);case DownloadSavesProgress_FileFinished():
return fileFinished(_that.name);case DownloadSavesProgress_Finished():
return finished();case DownloadSavesProgress_Cancelled():
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt totalFiles,  BigInt totalBytes)?  started,TResult? Function( String name,  String destination,  BigInt totalBytes)?  fileStarted,TResult? Function( BigInt downloadedBytes,  BigInt fileDownloadedBytes)?  progress,TResult? Function( String name)?  fileFinished,TResult? Function()?  finished,TResult? Function()?  cancelled,}) {final _that = this;
switch (_that) {
case DownloadSavesProgress_Started() when started != null:
return started(_that.totalFiles,_that.totalBytes);case DownloadSavesProgress_FileStarted() when fileStarted != null:
return fileStarted(_that.name,_that.destination,_that.totalBytes);case DownloadSavesProgress_Progress() when progress != null:
return progress(_that.downloadedBytes,_that.fileDownloadedBytes);case DownloadSavesProgress_FileFinished() when fileFinished != null:
return fileFinished(_that.name);case DownloadSavesProgress_Finished() when finished != null:
return finished();case DownloadSavesProgress_Cancelled() when cancelled != null:
return cancelled();case _:
  return null;

}
}

}

/// @nodoc


class DownloadSavesProgress_Started extends DownloadSavesProgress {
  const DownloadSavesProgress_Started({required this.totalFiles, required this.totalBytes}): super._();
  

 final  BigInt totalFiles;
 final  BigInt totalBytes;

/// Create a copy of DownloadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DownloadSavesProgress_StartedCopyWith<DownloadSavesProgress_Started> get copyWith => _$DownloadSavesProgress_StartedCopyWithImpl<DownloadSavesProgress_Started>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadSavesProgress_Started&&(identical(other.totalFiles, totalFiles) || other.totalFiles == totalFiles)&&(identical(other.totalBytes, totalBytes) || other.totalBytes == totalBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,totalFiles,totalBytes);
}

@override
String toString() {
    return 'DownloadSavesProgress.started(totalFiles: $totalFiles, totalBytes: $totalBytes)';
}


}

/// @nodoc
abstract mixin class $DownloadSavesProgress_StartedCopyWith<$Res> implements $DownloadSavesProgressCopyWith<$Res> {
  factory $DownloadSavesProgress_StartedCopyWith(DownloadSavesProgress_Started value, $Res Function(DownloadSavesProgress_Started) _then) = _$DownloadSavesProgress_StartedCopyWithImpl;
@useResult
$Res call({
 BigInt totalFiles, BigInt totalBytes
});




}
/// @nodoc
class _$DownloadSavesProgress_StartedCopyWithImpl<$Res>
    implements $DownloadSavesProgress_StartedCopyWith<$Res> {
  _$DownloadSavesProgress_StartedCopyWithImpl(this._self, this._then);

  final DownloadSavesProgress_Started _self;
  final $Res Function(DownloadSavesProgress_Started) _then;

/// Create a copy of DownloadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? totalFiles = null,Object? totalBytes = null,}) {
  return _then(DownloadSavesProgress_Started(
totalFiles: null == totalFiles ? _self.totalFiles : totalFiles // ignore: cast_nullable_to_non_nullable
as BigInt,totalBytes: null == totalBytes ? _self.totalBytes : totalBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class DownloadSavesProgress_FileStarted extends DownloadSavesProgress {
  const DownloadSavesProgress_FileStarted({required this.name, required this.destination, required this.totalBytes}): super._();
  

 final  String name;
 final  String destination;
 final  BigInt totalBytes;

/// Create a copy of DownloadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DownloadSavesProgress_FileStartedCopyWith<DownloadSavesProgress_FileStarted> get copyWith => _$DownloadSavesProgress_FileStartedCopyWithImpl<DownloadSavesProgress_FileStarted>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadSavesProgress_FileStarted&&(identical(other.name, name) || other.name == name)&&(identical(other.destination, destination) || other.destination == destination)&&(identical(other.totalBytes, totalBytes) || other.totalBytes == totalBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,name,destination,totalBytes);
}

@override
String toString() {
    return 'DownloadSavesProgress.fileStarted(name: $name, destination: $destination, totalBytes: $totalBytes)';
}


}

/// @nodoc
abstract mixin class $DownloadSavesProgress_FileStartedCopyWith<$Res> implements $DownloadSavesProgressCopyWith<$Res> {
  factory $DownloadSavesProgress_FileStartedCopyWith(DownloadSavesProgress_FileStarted value, $Res Function(DownloadSavesProgress_FileStarted) _then) = _$DownloadSavesProgress_FileStartedCopyWithImpl;
@useResult
$Res call({
 String name, String destination, BigInt totalBytes
});




}
/// @nodoc
class _$DownloadSavesProgress_FileStartedCopyWithImpl<$Res>
    implements $DownloadSavesProgress_FileStartedCopyWith<$Res> {
  _$DownloadSavesProgress_FileStartedCopyWithImpl(this._self, this._then);

  final DownloadSavesProgress_FileStarted _self;
  final $Res Function(DownloadSavesProgress_FileStarted) _then;

/// Create a copy of DownloadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,Object? destination = null,Object? totalBytes = null,}) {
  return _then(DownloadSavesProgress_FileStarted(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,destination: null == destination ? _self.destination : destination // ignore: cast_nullable_to_non_nullable
as String,totalBytes: null == totalBytes ? _self.totalBytes : totalBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class DownloadSavesProgress_Progress extends DownloadSavesProgress {
  const DownloadSavesProgress_Progress({required this.downloadedBytes, required this.fileDownloadedBytes}): super._();
  

 final  BigInt downloadedBytes;
 final  BigInt fileDownloadedBytes;

/// Create a copy of DownloadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DownloadSavesProgress_ProgressCopyWith<DownloadSavesProgress_Progress> get copyWith => _$DownloadSavesProgress_ProgressCopyWithImpl<DownloadSavesProgress_Progress>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadSavesProgress_Progress&&(identical(other.downloadedBytes, downloadedBytes) || other.downloadedBytes == downloadedBytes)&&(identical(other.fileDownloadedBytes, fileDownloadedBytes) || other.fileDownloadedBytes == fileDownloadedBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,downloadedBytes,fileDownloadedBytes);
}

@override
String toString() {
    return 'DownloadSavesProgress.progress(downloadedBytes: $downloadedBytes, fileDownloadedBytes: $fileDownloadedBytes)';
}


}

/// @nodoc
abstract mixin class $DownloadSavesProgress_ProgressCopyWith<$Res> implements $DownloadSavesProgressCopyWith<$Res> {
  factory $DownloadSavesProgress_ProgressCopyWith(DownloadSavesProgress_Progress value, $Res Function(DownloadSavesProgress_Progress) _then) = _$DownloadSavesProgress_ProgressCopyWithImpl;
@useResult
$Res call({
 BigInt downloadedBytes, BigInt fileDownloadedBytes
});




}
/// @nodoc
class _$DownloadSavesProgress_ProgressCopyWithImpl<$Res>
    implements $DownloadSavesProgress_ProgressCopyWith<$Res> {
  _$DownloadSavesProgress_ProgressCopyWithImpl(this._self, this._then);

  final DownloadSavesProgress_Progress _self;
  final $Res Function(DownloadSavesProgress_Progress) _then;

/// Create a copy of DownloadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? downloadedBytes = null,Object? fileDownloadedBytes = null,}) {
  return _then(DownloadSavesProgress_Progress(
downloadedBytes: null == downloadedBytes ? _self.downloadedBytes : downloadedBytes // ignore: cast_nullable_to_non_nullable
as BigInt,fileDownloadedBytes: null == fileDownloadedBytes ? _self.fileDownloadedBytes : fileDownloadedBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class DownloadSavesProgress_FileFinished extends DownloadSavesProgress {
  const DownloadSavesProgress_FileFinished({required this.name}): super._();
  

 final  String name;

/// Create a copy of DownloadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DownloadSavesProgress_FileFinishedCopyWith<DownloadSavesProgress_FileFinished> get copyWith => _$DownloadSavesProgress_FileFinishedCopyWithImpl<DownloadSavesProgress_FileFinished>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadSavesProgress_FileFinished&&(identical(other.name, name) || other.name == name));
}


@override
int get hashCode {
    return Object.hash(runtimeType,name);
}

@override
String toString() {
    return 'DownloadSavesProgress.fileFinished(name: $name)';
}


}

/// @nodoc
abstract mixin class $DownloadSavesProgress_FileFinishedCopyWith<$Res> implements $DownloadSavesProgressCopyWith<$Res> {
  factory $DownloadSavesProgress_FileFinishedCopyWith(DownloadSavesProgress_FileFinished value, $Res Function(DownloadSavesProgress_FileFinished) _then) = _$DownloadSavesProgress_FileFinishedCopyWithImpl;
@useResult
$Res call({
 String name
});




}
/// @nodoc
class _$DownloadSavesProgress_FileFinishedCopyWithImpl<$Res>
    implements $DownloadSavesProgress_FileFinishedCopyWith<$Res> {
  _$DownloadSavesProgress_FileFinishedCopyWithImpl(this._self, this._then);

  final DownloadSavesProgress_FileFinished _self;
  final $Res Function(DownloadSavesProgress_FileFinished) _then;

/// Create a copy of DownloadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,}) {
  return _then(DownloadSavesProgress_FileFinished(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class DownloadSavesProgress_Finished extends DownloadSavesProgress {
  const DownloadSavesProgress_Finished(): super._();
  






@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadSavesProgress_Finished);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'DownloadSavesProgress.finished()';
}


}




/// @nodoc


class DownloadSavesProgress_Cancelled extends DownloadSavesProgress {
  const DownloadSavesProgress_Cancelled(): super._();
  






@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is DownloadSavesProgress_Cancelled);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'DownloadSavesProgress.cancelled()';
}


}




// dart format on
