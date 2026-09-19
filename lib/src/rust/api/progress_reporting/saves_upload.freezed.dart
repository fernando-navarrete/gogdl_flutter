// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint, type=warning, deprecated_member_use, deprecated_member_use_from_same_package
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'saves_upload.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$UploadSavesProgress {





@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is UploadSavesProgress);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'UploadSavesProgress()';
}


}

/// @nodoc
class $UploadSavesProgressCopyWith<$Res>  {
$UploadSavesProgressCopyWith(UploadSavesProgress _, $Res Function(UploadSavesProgress) __);
}


/// Adds pattern-matching-related methods to [UploadSavesProgress].
extension UploadSavesProgressPatterns on UploadSavesProgress {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( UploadSavesProgress_Started value)?  started,TResult Function( UploadSavesProgress_FileStarted value)?  fileStarted,TResult Function( UploadSavesProgress_Progress value)?  progress,TResult Function( UploadSavesProgress_FileFinished value)?  fileFinished,TResult Function( UploadSavesProgress_Finished value)?  finished,required TResult orElse(),}){
final _that = this;
switch (_that) {
case UploadSavesProgress_Started() when started != null:
return started(_that);case UploadSavesProgress_FileStarted() when fileStarted != null:
return fileStarted(_that);case UploadSavesProgress_Progress() when progress != null:
return progress(_that);case UploadSavesProgress_FileFinished() when fileFinished != null:
return fileFinished(_that);case UploadSavesProgress_Finished() when finished != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( UploadSavesProgress_Started value)  started,required TResult Function( UploadSavesProgress_FileStarted value)  fileStarted,required TResult Function( UploadSavesProgress_Progress value)  progress,required TResult Function( UploadSavesProgress_FileFinished value)  fileFinished,required TResult Function( UploadSavesProgress_Finished value)  finished,}){
final _that = this;
switch (_that) {
case UploadSavesProgress_Started():
return started(_that);case UploadSavesProgress_FileStarted():
return fileStarted(_that);case UploadSavesProgress_Progress():
return progress(_that);case UploadSavesProgress_FileFinished():
return fileFinished(_that);case UploadSavesProgress_Finished():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( UploadSavesProgress_Started value)?  started,TResult? Function( UploadSavesProgress_FileStarted value)?  fileStarted,TResult? Function( UploadSavesProgress_Progress value)?  progress,TResult? Function( UploadSavesProgress_FileFinished value)?  fileFinished,TResult? Function( UploadSavesProgress_Finished value)?  finished,}){
final _that = this;
switch (_that) {
case UploadSavesProgress_Started() when started != null:
return started(_that);case UploadSavesProgress_FileStarted() when fileStarted != null:
return fileStarted(_that);case UploadSavesProgress_Progress() when progress != null:
return progress(_that);case UploadSavesProgress_FileFinished() when fileFinished != null:
return fileFinished(_that);case UploadSavesProgress_Finished() when finished != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt totalFiles)?  started,TResult Function( String name,  String source,  BigInt totalBytes)?  fileStarted,TResult Function( BigInt uploadedBytes,  BigInt fileUploadedBytes)?  progress,TResult Function( String name)?  fileFinished,TResult Function()?  finished,required TResult orElse(),}) {final _that = this;
switch (_that) {
case UploadSavesProgress_Started() when started != null:
return started(_that.totalFiles);case UploadSavesProgress_FileStarted() when fileStarted != null:
return fileStarted(_that.name,_that.source,_that.totalBytes);case UploadSavesProgress_Progress() when progress != null:
return progress(_that.uploadedBytes,_that.fileUploadedBytes);case UploadSavesProgress_FileFinished() when fileFinished != null:
return fileFinished(_that.name);case UploadSavesProgress_Finished() when finished != null:
return finished();case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt totalFiles)  started,required TResult Function( String name,  String source,  BigInt totalBytes)  fileStarted,required TResult Function( BigInt uploadedBytes,  BigInt fileUploadedBytes)  progress,required TResult Function( String name)  fileFinished,required TResult Function()  finished,}) {final _that = this;
switch (_that) {
case UploadSavesProgress_Started():
return started(_that.totalFiles);case UploadSavesProgress_FileStarted():
return fileStarted(_that.name,_that.source,_that.totalBytes);case UploadSavesProgress_Progress():
return progress(_that.uploadedBytes,_that.fileUploadedBytes);case UploadSavesProgress_FileFinished():
return fileFinished(_that.name);case UploadSavesProgress_Finished():
return finished();}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt totalFiles)?  started,TResult? Function( String name,  String source,  BigInt totalBytes)?  fileStarted,TResult? Function( BigInt uploadedBytes,  BigInt fileUploadedBytes)?  progress,TResult? Function( String name)?  fileFinished,TResult? Function()?  finished,}) {final _that = this;
switch (_that) {
case UploadSavesProgress_Started() when started != null:
return started(_that.totalFiles);case UploadSavesProgress_FileStarted() when fileStarted != null:
return fileStarted(_that.name,_that.source,_that.totalBytes);case UploadSavesProgress_Progress() when progress != null:
return progress(_that.uploadedBytes,_that.fileUploadedBytes);case UploadSavesProgress_FileFinished() when fileFinished != null:
return fileFinished(_that.name);case UploadSavesProgress_Finished() when finished != null:
return finished();case _:
  return null;

}
}

}

/// @nodoc


class UploadSavesProgress_Started extends UploadSavesProgress {
  const UploadSavesProgress_Started({required this.totalFiles}): super._();
  

 final  BigInt totalFiles;

/// Create a copy of UploadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UploadSavesProgress_StartedCopyWith<UploadSavesProgress_Started> get copyWith => _$UploadSavesProgress_StartedCopyWithImpl<UploadSavesProgress_Started>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is UploadSavesProgress_Started&&(identical(other.totalFiles, totalFiles) || other.totalFiles == totalFiles));
}


@override
int get hashCode {
    return Object.hash(runtimeType,totalFiles);
}

@override
String toString() {
    return 'UploadSavesProgress.started(totalFiles: $totalFiles)';
}


}

/// @nodoc
abstract mixin class $UploadSavesProgress_StartedCopyWith<$Res> implements $UploadSavesProgressCopyWith<$Res> {
  factory $UploadSavesProgress_StartedCopyWith(UploadSavesProgress_Started value, $Res Function(UploadSavesProgress_Started) _then) = _$UploadSavesProgress_StartedCopyWithImpl;
@useResult
$Res call({
 BigInt totalFiles
});




}
/// @nodoc
class _$UploadSavesProgress_StartedCopyWithImpl<$Res>
    implements $UploadSavesProgress_StartedCopyWith<$Res> {
  _$UploadSavesProgress_StartedCopyWithImpl(this._self, this._then);

  final UploadSavesProgress_Started _self;
  final $Res Function(UploadSavesProgress_Started) _then;

/// Create a copy of UploadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? totalFiles = null,}) {
  return _then(UploadSavesProgress_Started(
totalFiles: null == totalFiles ? _self.totalFiles : totalFiles // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class UploadSavesProgress_FileStarted extends UploadSavesProgress {
  const UploadSavesProgress_FileStarted({required this.name, required this.source, required this.totalBytes}): super._();
  

 final  String name;
 final  String source;
 final  BigInt totalBytes;

/// Create a copy of UploadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UploadSavesProgress_FileStartedCopyWith<UploadSavesProgress_FileStarted> get copyWith => _$UploadSavesProgress_FileStartedCopyWithImpl<UploadSavesProgress_FileStarted>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is UploadSavesProgress_FileStarted&&(identical(other.name, name) || other.name == name)&&(identical(other.source, source) || other.source == source)&&(identical(other.totalBytes, totalBytes) || other.totalBytes == totalBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,name,source,totalBytes);
}

@override
String toString() {
    return 'UploadSavesProgress.fileStarted(name: $name, source: $source, totalBytes: $totalBytes)';
}


}

/// @nodoc
abstract mixin class $UploadSavesProgress_FileStartedCopyWith<$Res> implements $UploadSavesProgressCopyWith<$Res> {
  factory $UploadSavesProgress_FileStartedCopyWith(UploadSavesProgress_FileStarted value, $Res Function(UploadSavesProgress_FileStarted) _then) = _$UploadSavesProgress_FileStartedCopyWithImpl;
@useResult
$Res call({
 String name, String source, BigInt totalBytes
});




}
/// @nodoc
class _$UploadSavesProgress_FileStartedCopyWithImpl<$Res>
    implements $UploadSavesProgress_FileStartedCopyWith<$Res> {
  _$UploadSavesProgress_FileStartedCopyWithImpl(this._self, this._then);

  final UploadSavesProgress_FileStarted _self;
  final $Res Function(UploadSavesProgress_FileStarted) _then;

/// Create a copy of UploadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,Object? source = null,Object? totalBytes = null,}) {
  return _then(UploadSavesProgress_FileStarted(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,source: null == source ? _self.source : source // ignore: cast_nullable_to_non_nullable
as String,totalBytes: null == totalBytes ? _self.totalBytes : totalBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class UploadSavesProgress_Progress extends UploadSavesProgress {
  const UploadSavesProgress_Progress({required this.uploadedBytes, required this.fileUploadedBytes}): super._();
  

 final  BigInt uploadedBytes;
 final  BigInt fileUploadedBytes;

/// Create a copy of UploadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UploadSavesProgress_ProgressCopyWith<UploadSavesProgress_Progress> get copyWith => _$UploadSavesProgress_ProgressCopyWithImpl<UploadSavesProgress_Progress>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is UploadSavesProgress_Progress&&(identical(other.uploadedBytes, uploadedBytes) || other.uploadedBytes == uploadedBytes)&&(identical(other.fileUploadedBytes, fileUploadedBytes) || other.fileUploadedBytes == fileUploadedBytes));
}


@override
int get hashCode {
    return Object.hash(runtimeType,uploadedBytes,fileUploadedBytes);
}

@override
String toString() {
    return 'UploadSavesProgress.progress(uploadedBytes: $uploadedBytes, fileUploadedBytes: $fileUploadedBytes)';
}


}

/// @nodoc
abstract mixin class $UploadSavesProgress_ProgressCopyWith<$Res> implements $UploadSavesProgressCopyWith<$Res> {
  factory $UploadSavesProgress_ProgressCopyWith(UploadSavesProgress_Progress value, $Res Function(UploadSavesProgress_Progress) _then) = _$UploadSavesProgress_ProgressCopyWithImpl;
@useResult
$Res call({
 BigInt uploadedBytes, BigInt fileUploadedBytes
});




}
/// @nodoc
class _$UploadSavesProgress_ProgressCopyWithImpl<$Res>
    implements $UploadSavesProgress_ProgressCopyWith<$Res> {
  _$UploadSavesProgress_ProgressCopyWithImpl(this._self, this._then);

  final UploadSavesProgress_Progress _self;
  final $Res Function(UploadSavesProgress_Progress) _then;

/// Create a copy of UploadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? uploadedBytes = null,Object? fileUploadedBytes = null,}) {
  return _then(UploadSavesProgress_Progress(
uploadedBytes: null == uploadedBytes ? _self.uploadedBytes : uploadedBytes // ignore: cast_nullable_to_non_nullable
as BigInt,fileUploadedBytes: null == fileUploadedBytes ? _self.fileUploadedBytes : fileUploadedBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class UploadSavesProgress_FileFinished extends UploadSavesProgress {
  const UploadSavesProgress_FileFinished({required this.name}): super._();
  

 final  String name;

/// Create a copy of UploadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UploadSavesProgress_FileFinishedCopyWith<UploadSavesProgress_FileFinished> get copyWith => _$UploadSavesProgress_FileFinishedCopyWithImpl<UploadSavesProgress_FileFinished>(this, _$identity);



@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is UploadSavesProgress_FileFinished&&(identical(other.name, name) || other.name == name));
}


@override
int get hashCode {
    return Object.hash(runtimeType,name);
}

@override
String toString() {
    return 'UploadSavesProgress.fileFinished(name: $name)';
}


}

/// @nodoc
abstract mixin class $UploadSavesProgress_FileFinishedCopyWith<$Res> implements $UploadSavesProgressCopyWith<$Res> {
  factory $UploadSavesProgress_FileFinishedCopyWith(UploadSavesProgress_FileFinished value, $Res Function(UploadSavesProgress_FileFinished) _then) = _$UploadSavesProgress_FileFinishedCopyWithImpl;
@useResult
$Res call({
 String name
});




}
/// @nodoc
class _$UploadSavesProgress_FileFinishedCopyWithImpl<$Res>
    implements $UploadSavesProgress_FileFinishedCopyWith<$Res> {
  _$UploadSavesProgress_FileFinishedCopyWithImpl(this._self, this._then);

  final UploadSavesProgress_FileFinished _self;
  final $Res Function(UploadSavesProgress_FileFinished) _then;

/// Create a copy of UploadSavesProgress
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? name = null,}) {
  return _then(UploadSavesProgress_FileFinished(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class UploadSavesProgress_Finished extends UploadSavesProgress {
  const UploadSavesProgress_Finished(): super._();
  






@override
bool operator ==(Object other) {
    return identical(this, other) || (other.runtimeType == runtimeType&&other is UploadSavesProgress_Finished);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
    return 'UploadSavesProgress.finished()';
}


}




// dart format on
